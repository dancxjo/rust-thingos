#!/usr/bin/env python3
import subprocess
import time
import sys
import os
import signal
import select
import json
import math
import socket
import tempfile
import threading
from datetime import datetime

# Configuration
TIMEOUT = float(os.environ.get("HUNTER_TIMEOUT", "15.0"))
ARCH = os.environ.get("KARCH", "x86_64")
LOG_DIR = "freeze_logs"
LOG_LEVEL = os.environ.get("HUNTER_LOGLEVEL", "5") # 5 = Trace
MODE = os.environ.get("HUNTER_MODE", os.environ.get("FREEZE_HUNTER_MODE", "idle"))
MOUSE_INTERVAL = float(os.environ.get("HUNTER_MOUSE_INTERVAL", "0.20"))
MOUSE_RADIUS = int(os.environ.get("HUNTER_MOUSE_RADIUS", "32"))
MOUSE_STEPS = int(os.environ.get("HUNTER_MOUSE_STEPS", "32"))
ALT_TAB_INTERVAL = float(os.environ.get("HUNTER_ALT_TAB_INTERVAL", "4.0"))
QMP_CONNECT_TIMEOUT = float(os.environ.get("HUNTER_QMP_CONNECT_TIMEOUT", "15.0"))

MODE_ALIASES = {
    "idle": set(),
    "none": set(),
    "passive": set(),
    "mouse": {"mouse-circle"},
    "cursor": {"mouse-circle"},
    "circle": {"mouse-circle"},
    "mouse-circle": {"mouse-circle"},
    "alt-tab": {"alt-tab"},
    "alttab": {"alt-tab"},
    "switch": {"alt-tab"},
    "exercise": {"mouse-circle", "alt-tab"},
    "active": {"mouse-circle", "alt-tab"},
    "desktop": {"mouse-circle", "alt-tab"},
}

def parse_modes(value):
    modes = set()
    for raw_part in value.replace("+", ",").split(","):
        part = raw_part.strip().lower().replace("_", "-")
        if not part:
            continue
        if part not in MODE_ALIASES:
            known = ", ".join(sorted(MODE_ALIASES))
            raise ValueError(f"unknown HUNTER_MODE '{raw_part}' (known: {known})")
        modes.update(MODE_ALIASES[part])
    return modes

def compose_qemu_flags(base_flags, qmp_path=None):
    flags = base_flags or "-m 2G -smp 4"
    if "-display" not in flags:
        flags += " -display none"
    if qmp_path and "-qmp" not in flags:
        flags += f" -qmp unix:{qmp_path},server=on,wait=off"
    return flags

def build_iteration(session_id):
    print(f"[*] Building ISO for session {session_id:04d}...")
    subprocess.run(["just", "iso", ARCH], check=True)

class QmpInputDriver:
    def __init__(self, socket_path, modes):
        self.socket_path = socket_path
        self.modes = modes
        self.stop_event = threading.Event()
        self.thread = threading.Thread(target=self.run, daemon=True)

    def start(self):
        self.thread.start()

    def stop(self):
        self.stop_event.set()
        self.thread.join(timeout=1.0)

    def run(self):
        deadline = time.time() + QMP_CONNECT_TIMEOUT
        while not self.stop_event.is_set() and time.time() < deadline:
            try:
                with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as qmp:
                    qmp.settimeout(1.0)
                    qmp.connect(self.socket_path)
                    self._read_qmp_line(qmp)
                    self._execute(qmp, {"execute": "qmp_capabilities"})
                    self._drive(qmp)
                    return
            except (FileNotFoundError, ConnectionRefusedError, socket.timeout, OSError):
                time.sleep(0.1)

    def _drive(self, qmp):
        next_mouse = time.time()
        next_alt_tab = time.time() + min(ALT_TAB_INTERVAL, 1.0)
        step = 0
        prev_x = MOUSE_RADIUS
        prev_y = 0

        while not self.stop_event.is_set():
            now = time.time()
            if "mouse-circle" in self.modes and now >= next_mouse:
                angle = ((step + 1) / max(MOUSE_STEPS, 1)) * math.tau
                x = round(math.cos(angle) * MOUSE_RADIUS)
                y = round(math.sin(angle) * MOUSE_RADIUS)
                self._send_rel(qmp, x - prev_x, y - prev_y)
                prev_x = x
                prev_y = y
                step = (step + 1) % max(MOUSE_STEPS, 1)
                next_mouse = now + max(MOUSE_INTERVAL, 0.01)

            if "alt-tab" in self.modes and now >= next_alt_tab:
                self._send_alt_tab(qmp)
                next_alt_tab = now + max(ALT_TAB_INTERVAL, 0.1)

            self.stop_event.wait(0.02)

    def _send_rel(self, qmp, dx, dy):
        events = []
        if dx:
            events.append({"type": "rel", "data": {"axis": "x", "value": dx}})
        if dy:
            events.append({"type": "rel", "data": {"axis": "y", "value": dy}})
        if events:
            self._execute(qmp, {"execute": "input-send-event", "arguments": {"events": events}})

    def _send_alt_tab(self, qmp):
        for key, down in (("alt", True), ("tab", True), ("tab", False), ("alt", False)):
            self._execute(qmp, {
                "execute": "input-send-event",
                "arguments": {
                    "events": [{
                        "type": "key",
                        "data": {"down": down, "key": {"type": "qcode", "data": key}},
                    }],
                },
            })
            time.sleep(0.03)

    def _execute(self, qmp, command):
        qmp.sendall(json.dumps(command).encode("utf-8") + b"\n")
        while not self.stop_event.is_set():
            response = self._read_qmp_line(qmp)
            if response is None or "event" in response:
                continue
            return response
        return None

    def _read_qmp_line(self, qmp):
        chunks = []
        while not self.stop_event.is_set():
            chunk = qmp.recv(1)
            if not chunk:
                raise OSError("QMP connection closed")
            if chunk == b"\n":
                break
            chunks.append(chunk)
        if not chunks:
            return None
        return json.loads(b"".join(chunks).decode("utf-8"))

def stop_process_group(process, *, sig=signal.SIGTERM, grace=2.0):
    if process.poll() is not None:
        return

    try:
        pgid = os.getpgid(process.pid)
    except ProcessLookupError:
        return

    try:
        os.killpg(pgid, sig)
    except ProcessLookupError:
        return

    deadline = time.time() + grace
    while time.time() < deadline:
        if process.poll() is not None:
            return
        time.sleep(0.05)

    try:
        os.killpg(pgid, signal.SIGKILL)
    except ProcessLookupError:
        pass

def run_session(session_id):
    modes = parse_modes(MODE)
    os.makedirs(LOG_DIR, exist_ok=True)
    timestamp = datetime.now().strftime("%H%M%S")
    log_path = os.path.join(LOG_DIR, f"run_{session_id:04d}_{timestamp}.log")

    # Use just run. It handles building and standard environment setup (audio, etc).
    # The default display stays headless for terminal log capture.
    cmd = [
        "just", "run", ARCH, "--loglevel", LOG_LEVEL
    ]

    print(f"[*] Session {session_id:04d} | Log: {log_path}", end="\r")

    env = os.environ.copy()
    qmp_dir = None
    qmp_input = None
    qmp_path = None
    if modes:
        qmp_dir = tempfile.TemporaryDirectory(prefix=f"freeze-hunter-{session_id:04d}-")
        qmp_path = os.path.join(qmp_dir.name, "qmp.sock")
    env["QEMUFLAGS"] = compose_qemu_flags(env.get("QEMUFLAGS", "-m 2G -smp 4"), qmp_path)

    with open(log_path, "w") as log_file:
        # Start in a new process group so we can kill all children (QEMU)
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            stdin=subprocess.DEVNULL,
            env=env,
            preexec_fn=os.setsid
        )
        os.set_blocking(process.stdout.fileno(), False)

        last_output_time = time.time()
        start_time = time.time()
        byte_count = 0
        if modes:
            qmp_input = QmpInputDriver(qmp_path, modes)
            qmp_input.start()

        try:
            while True:
                rlist, _, _ = select.select([process.stdout], [], [], 0.1)

                if rlist:
                    try:
                        chunk = os.read(process.stdout.fileno(), 65536)
                    except BlockingIOError:
                        chunk = b""

                    if not chunk:
                        break

                    text = chunk.decode(errors="replace")
                    log_file.write(text)
                    log_file.flush()
                    byte_count += len(chunk)
                    last_output_time = time.time()

                # Silence check
                silence = time.time() - last_output_time
                if silence > TIMEOUT:
                    print(f"\n[!] SILENCE DETECTED ({silence:.1f}s) in session {session_id}. Killing.")
                    log_file.write(f"\n\n[!!!] KILLED DUE TO SILENCE AT {datetime.now().isoformat()}\n")
                    break

                # Check if process exited early
                if process.poll() is not None:
                    break

        except KeyboardInterrupt:
            print(f"\n[*] Interrupt received; stopping session {session_id}.")
            stop_process_group(process, sig=signal.SIGTERM, grace=1.0)
            raise
        finally:
            stop_process_group(process, sig=signal.SIGTERM)
            if qmp_input:
                qmp_input.stop()
            process.stdout.close()
            if qmp_dir:
                qmp_dir.cleanup()

    runtime = time.time() - start_time
    # If it died very quickly with almost no output, it might be a build error or config issue
    if byte_count < 200 and runtime < 1.0:
        print(f"\n[!] Session {session_id} failed to produce output. Check build or QEMU logs.")
        return False

    return True

def main():
    modes = parse_modes(MODE)
    print(f"=== ThingOS Freeze Hunter ===")
    print(f"[*] Arch:    {ARCH}")
    print(f"[*] Timeout: {TIMEOUT}s")
    print(f"[*] Level:   {LOG_LEVEL}")
    print(f"[*] Mode:    {MODE} ({', '.join(sorted(modes)) if modes else 'passive'})")
    print(f"[*] Logs:    {LOG_DIR}/")
    print("[*] Ctrl+C to stop.")

    try:
        session_id = 0
        while True:
            build_iteration(session_id)
            if not run_session(session_id):
                print("[*] Cooling down before retry (5s)...")
                time.sleep(5)
                continue
            session_id += 1
            # Brief pause between runs
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n[*] Stopped by user.")
    except Exception as e:
        print(f"\n[!] Fatal error: {e}")

if __name__ == "__main__":
    main()
