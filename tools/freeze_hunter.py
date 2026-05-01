#!/usr/bin/env python3
import subprocess
import time
import sys
import os
import signal
import select
import json
import math
import random
import socket
import tempfile
import threading
import re
from dataclasses import dataclass
from datetime import datetime

# Configuration
TIMEOUT = float(os.environ.get("HUNTER_TIMEOUT", "15.0"))
ARCH = os.environ.get("KARCH", "x86_64")
LOG_DIR = "freeze_logs"
LOG_LEVEL = os.environ.get("HUNTER_LOGLEVEL", "5") # 5 = Trace
MODE = os.environ.get("HUNTER_MODE", os.environ.get("FREEZE_HUNTER_MODE", "stress"))
RESOLUTION = os.environ.get("HUNTER_RESOLUTION", os.environ.get("BDD_RESOLUTION", "1920x1080"))
MOUSE_INTERVAL = float(os.environ.get("HUNTER_MOUSE_INTERVAL", "0.20"))
MOUSE_RADIUS = int(os.environ.get("HUNTER_MOUSE_RADIUS", "32"))
MOUSE_STEPS = int(os.environ.get("HUNTER_MOUSE_STEPS", "32"))
ALT_TAB_INTERVAL = float(os.environ.get("HUNTER_ALT_TAB_INTERVAL", "4.0"))
RANDOM_INPUT_INTERVAL = float(os.environ.get("HUNTER_RANDOM_INPUT_INTERVAL", "0.35"))
RANDOM_MOUSE_RADIUS = int(os.environ.get("HUNTER_RANDOM_MOUSE_RADIUS", "96"))
DESKTOP_ACTION_INTERVAL = float(os.environ.get("HUNTER_DESKTOP_ACTION_INTERVAL", "1.25"))
QMP_CONNECT_TIMEOUT = float(os.environ.get("HUNTER_QMP_CONNECT_TIMEOUT", "15.0"))
SERIAL_COMMAND_SETTLE = float(os.environ.get("HUNTER_SERIAL_COMMAND_SETTLE", "0.45"))
RANDOM_SEED = os.environ.get("HUNTER_SEED")

ACTIVE_MODES = {"mouse-circle", "alt-tab"}
DESKTOP_MODES = {"desktop-actions"}
STRESS_MODES = ACTIVE_MODES | {"random-input"} | DESKTOP_MODES

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
    "random": {"random-input"},
    "random-input": {"random-input"},
    "fuzz": {"random-input"},
    "bdd": {"desktop-actions"},
    "bdd-steps": {"desktop-actions"},
    "desktop-actions": {"desktop-actions"},
    "wayland-actions": {"desktop-actions"},
    "user-actions": {"desktop-actions"},
    "exercise": ACTIVE_MODES,
    "active": ACTIVE_MODES,
    "desktop": ACTIVE_MODES | DESKTOP_MODES,
    "stress": STRESS_MODES,
    "default": STRESS_MODES,
}

RANDOM_KEY_CODES = [
    "up", "down", "left", "right",
    "pgup", "pgdn", "home", "end",
    "esc", "spc", "ret", "backspace",
    "a", "b", "c", "d", "e", "f",
    "tab",
]

WINDOW_RE = re.compile(
    r'(?m)^\s*\d+\s+(\d+)x(\d+)\+(-?\d+),(-?\d+)\s+z=(-?\d+)\s+title="([^"]*)"'
)
ANSI_RE = re.compile(r"\x1b\[[0-?]*[ -/]*[@-~]")

LAUNCH_COMMANDS = [
    "/bin/wayland_hello &",
    "/bin/clock &",
    "/bin/leaf &",
    "ps",
    "cat /session/wayland/windows/index",
    "cat /session/wayland/events/latest",
    "cat /session/wayland/components",
]

def parse_resolution(value):
    try:
        width, height = value.lower().split("x", 1)
        return max(1, int(width)), max(1, int(height))
    except (TypeError, ValueError):
        return 1920, 1080

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

def strip_ansi(text):
    return ANSI_RE.sub("", text)

@dataclass
class WindowInfo:
    w: int
    h: int
    x: int
    y: int
    z: int
    title: str

    @property
    def right(self):
        return self.x + self.w

    @property
    def bottom(self):
        return self.y + self.h

class SerialLogBuffer:
    def __init__(self):
        self._lock = threading.Lock()
        self._text = ""

    def append(self, text):
        with self._lock:
            self._text += text

    def snapshot(self):
        with self._lock:
            return self._text

    def offset(self):
        with self._lock:
            return len(self._text)

    def since(self, offset):
        with self._lock:
            return self._text[min(offset, len(self._text)):]

    def contains(self, needle):
        return needle.lower() in strip_ansi(self.snapshot()).lower()

class SerialConsole:
    def __init__(self, stdin, serial_log):
        self.stdin = stdin
        self.serial_log = serial_log
        self._lock = threading.Lock()

    def write_line(self, command):
        if self.stdin is None or self.stdin.closed:
            return None
        offset = self.serial_log.offset()
        data = (command.rstrip("\n") + "\n").encode("utf-8")
        with self._lock:
            try:
                for byte in data:
                    self.stdin.write(bytes([byte]))
                    self.stdin.flush()
                    time.sleep(0.004)
            except (BrokenPipeError, OSError, ValueError):
                return None
        return offset

    def command_output(self, command, settle=SERIAL_COMMAND_SETTLE):
        offset = self.write_line(command)
        if offset is None:
            return ""
        time.sleep(max(settle, 0.01))
        return strip_ansi(self.serial_log.since(offset))

@dataclass(frozen=True)
class HunterAction:
    name: str
    weight: int
    run: object

class QmpInputDriver:
    def __init__(self, socket_path, modes, serial_console=None, serial_log=None):
        self.socket_path = socket_path
        self.modes = modes
        self.serial_console = serial_console
        self.serial_log = serial_log
        self.screen_width, self.screen_height = parse_resolution(RESOLUTION)
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
            except (FileNotFoundError, ConnectionRefusedError, socket.timeout, OSError, json.JSONDecodeError) as e:
                if self.stop_event.is_set():
                    return
                if time.time() >= deadline:
                    print(f"\n[!] QMP input driver could not connect: {e}")
                    return
                time.sleep(0.1)

    def _drive(self, qmp):
        next_mouse = time.time()
        next_alt_tab = time.time() + min(ALT_TAB_INTERVAL, 1.0)
        next_random = time.time() + random.uniform(0.05, max(RANDOM_INPUT_INTERVAL, 0.05))
        next_desktop = time.time() + random.uniform(0.5, max(DESKTOP_ACTION_INTERVAL, 0.5))
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

            if "random-input" in self.modes and now >= next_random:
                self._send_random_input(qmp)
                interval = max(RANDOM_INPUT_INTERVAL, 0.05)
                next_random = now + random.uniform(interval * 0.5, interval * 1.5)

            if "desktop-actions" in self.modes and now >= next_desktop:
                self._send_desktop_action(qmp)
                interval = max(DESKTOP_ACTION_INTERVAL, 0.10)
                next_desktop = now + random.uniform(interval * 0.5, interval * 1.6)

            self.stop_event.wait(0.02)

    def _send_rel(self, qmp, dx, dy):
        events = []
        if dx:
            events.append({"type": "rel", "data": {"axis": "x", "value": dx}})
        if dy:
            events.append({"type": "rel", "data": {"axis": "y", "value": dy}})
        if events:
            self._execute(qmp, {"execute": "input-send-event", "arguments": {"events": events}})

    def _move_to(self, qmp, x, y):
        x = int(max(0, min(self.screen_width - 1, x)))
        y = int(max(0, min(self.screen_height - 1, y)))
        self._send_rel(qmp, -10000, -10000)
        time.sleep(0.04)
        self._send_rel(qmp, x, y)

    def _drag_from_to(self, qmp, start_x, start_y, end_x, end_y, steps=None):
        steps = steps or random.randint(3, 8)
        self._move_to(qmp, start_x, start_y)
        time.sleep(random.uniform(0.03, 0.12))
        self._send_button(qmp, "left", True)
        last_x = start_x
        last_y = start_y
        for i in range(1, steps + 1):
            next_x = round(start_x + ((end_x - start_x) * i / steps))
            next_y = round(start_y + ((end_y - start_y) * i / steps))
            self._send_rel(qmp, next_x - last_x, next_y - last_y)
            last_x = next_x
            last_y = next_y
            time.sleep(random.uniform(0.025, 0.10))
        self._send_button(qmp, "left", False)

    def _send_alt_tab(self, qmp):
        self._send_combo(qmp, ["alt"], "tab")

    def _send_random_input(self, qmp):
        action = random.choice(("move", "click", "key", "wheel"))
        if action == "move":
            radius = max(RANDOM_MOUSE_RADIUS, 1)
            self._send_rel(qmp, random.randint(-radius, radius), random.randint(-radius, radius))
        elif action == "click":
            button = random.choice(("left", "right", "middle"))
            self._send_button(qmp, button, True)
            time.sleep(random.uniform(0.01, 0.05))
            self._send_button(qmp, button, False)
        elif action == "key":
            key = random.choice(RANDOM_KEY_CODES)
            self._send_key(qmp, key, True)
            time.sleep(random.uniform(0.01, 0.05))
            self._send_key(qmp, key, False)
        else:
            self._send_rel(qmp, 0, random.choice((-3, -2, 2, 3)))

    def _send_desktop_action(self, qmp):
        actions = self._desktop_actions()
        total = sum(action.weight for action in actions)
        pick = random.uniform(0, total)
        upto = 0
        for action in actions:
            upto += action.weight
            if pick <= upto:
                try:
                    detail = action.run(qmp)
                    if detail:
                        print(f"\n[*] hunter action: {action.name} ({detail})")
                    else:
                        print(f"\n[*] hunter action: {action.name}")
                except (OSError, socket.timeout):
                    raise
                except Exception as e:
                    print(f"\n[!] hunter action failed: {action.name}: {e}")
                return

    def _desktop_actions(self):
        return [
            HunterAction("drag-window", 18, self._action_drag_window),
            HunterAction("resize-window", 14, self._action_resize_window),
            HunterAction("chrome-button", 12, self._action_chrome_button),
            HunterAction("click-and-type", 12, self._action_click_and_type),
            HunterAction("drag-clock-over-hello", 7, self._action_drag_clock_over_hello),
            HunterAction("launch-or-query", 16, self._action_launch_or_query),
            HunterAction("alt-tab", 10, self._action_alt_tab),
            HunterAction("random-pointer", 11, self._action_random_pointer),
        ]

    def _read_windows(self):
        text = ""
        if self.serial_console:
            text = self.serial_console.command_output("cat /session/wayland/windows/index")
        if not text and self.serial_log:
            text = self.serial_log.snapshot()
        windows = [
            WindowInfo(
                w=int(c.group(1)),
                h=int(c.group(2)),
                x=int(c.group(3)),
                y=int(c.group(4)),
                z=int(c.group(5)),
                title=c.group(6),
            )
            for c in WINDOW_RE.finditer(strip_ansi(text))
        ]
        deduped = {}
        for window in windows:
            deduped[(window.title, window.x, window.y, window.w, window.h)] = window
        return sorted(deduped.values(), key=lambda w: w.z)

    def _interesting_windows(self):
        windows = self._read_windows()
        if windows:
            return windows
        # Fallback geometry mirrors the BDD blossom recipes while the shell
        # index is unavailable or still booting.
        return [WindowInfo(480, 320, 0, 0, 0, "fallback Wayland hello")]

    def _pick_window(self, preferred=None):
        windows = self._interesting_windows()
        if preferred:
            matches = [w for w in windows if preferred.lower() in w.title.lower()]
            if matches:
                return random.choice(matches)
        return random.choice(windows)

    def _action_drag_window(self, qmp):
        window = self._pick_window()
        start_x = window.x + random.randint(20, max(21, min(window.w - 20, 140)))
        start_y = window.y + random.randint(10, 24)
        end_x = start_x + random.randint(-260, 260)
        end_y = start_y + random.randint(-180, 180)
        self._drag_from_to(qmp, start_x, start_y, end_x, end_y)
        return window.title

    def _action_resize_window(self, qmp):
        window = self._pick_window("Thing-OS Wayland Lab")
        start_x = window.right - random.randint(4, 18)
        start_y = window.bottom - random.randint(4, 18)
        end_x = start_x + random.randint(-120, 220)
        end_y = start_y + random.randint(-90, 180)
        self._drag_from_to(qmp, start_x, start_y, end_x, end_y)
        return window.title

    def _action_chrome_button(self, qmp):
        window = self._pick_window("Thing-OS Wayland Lab")
        button = random.choice(("minimize", "maximize", "close"))
        offsets = {"minimize": 86, "maximize": 52, "close": 20}
        x = window.right - offsets[button]
        y = window.y + 20
        self._move_to(qmp, x, y)
        self._send_button(qmp, "left", True)
        time.sleep(random.uniform(0.04, 0.15))
        self._send_button(qmp, "left", False)
        return f"{button} {window.title}"

    def _action_click_and_type(self, qmp):
        window = self._pick_window("Thing-OS Wayland Lab")
        x = window.x + min(max(window.w // 3, 80), max(window.w - 20, 80))
        y = window.y + min(max(window.h // 3, 70), max(window.h - 20, 70))
        self._move_to(qmp, x, y)
        self._send_button(qmp, "left", True)
        time.sleep(0.04)
        self._send_button(qmp, "left", False)
        for key in random.sample(["a", "b", "c", "d", "e", "f", "spc", "backspace"], k=3):
            self._send_key_tap(qmp, key)
            time.sleep(random.uniform(0.02, 0.08))
        return window.title

    def _action_drag_clock_over_hello(self, qmp):
        windows = self._interesting_windows()
        hello = next((w for w in windows if "Wayland Lab" in w.title), None)
        clock = next((w for w in windows if "Clock" in w.title), None)
        if not hello or not clock:
            return self._action_drag_window(qmp)
        start_x = clock.x + 24
        start_y = clock.y + 14
        target_x = hello.x + 40
        target_y = max(0, hello.y - 20)
        self._drag_from_to(qmp, start_x, start_y, target_x, target_y, steps=5)
        return "Clock -> Wayland hello"

    def _action_launch_or_query(self, _qmp):
        if not self.serial_console:
            return ""
        command = random.choice(LAUNCH_COMMANDS)
        self.serial_console.write_line(command)
        return command

    def _action_alt_tab(self, qmp):
        count = random.randint(1, 4)
        for _ in range(count):
            self._send_alt_tab(qmp)
            time.sleep(random.uniform(0.05, 0.25))
        return f"{count} cycles"

    def _action_random_pointer(self, qmp):
        for _ in range(random.randint(2, 7)):
            self._send_random_input(qmp)
            time.sleep(random.uniform(0.02, 0.12))
        return ""

    def _send_key(self, qmp, key, down):
        self._execute(qmp, {
            "execute": "input-send-event",
            "arguments": {
                "events": [{
                    "type": "key",
                    "data": {"down": down, "key": {"type": "qcode", "data": key}},
                }],
            },
        })

    def _send_key_tap(self, qmp, key):
        self._send_key(qmp, key, True)
        time.sleep(random.uniform(0.01, 0.05))
        self._send_key(qmp, key, False)

    def _send_combo(self, qmp, modifiers, key):
        for modifier in modifiers:
            self._send_key(qmp, modifier, True)
            time.sleep(0.02)
        self._send_key_tap(qmp, key)
        for modifier in reversed(modifiers):
            time.sleep(0.02)
            self._send_key(qmp, modifier, False)

    def _send_button(self, qmp, button, down):
        self._execute(qmp, {
            "execute": "input-send-event",
            "arguments": {
                "events": [{
                    "type": "btn",
                    "data": {"down": down, "button": button},
                }],
            },
        })

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
            try:
                chunk = qmp.recv(1)
            except socket.timeout:
                raise
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
    serial_log = SerialLogBuffer()

    with open(log_path, "w") as log_file:
        # Start in a new process group so we can kill all children (QEMU)
        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            stdin=subprocess.PIPE,
            env=env,
            preexec_fn=os.setsid
        )
        os.set_blocking(process.stdout.fileno(), False)

        last_output_time = time.time()
        start_time = time.time()
        byte_count = 0
        if modes:
            serial_console = SerialConsole(process.stdin, serial_log)
            qmp_input = QmpInputDriver(qmp_path, modes, serial_console, serial_log)
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
                    serial_log.append(text)
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
            if process.stdin:
                process.stdin.close()
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
    if RANDOM_SEED is not None:
        random.seed(RANDOM_SEED)
    modes = parse_modes(MODE)
    print(f"=== ThingOS Freeze Hunter ===")
    print(f"[*] Arch:    {ARCH}")
    print(f"[*] Timeout: {TIMEOUT}s")
    print(f"[*] Level:   {LOG_LEVEL}")
    print(f"[*] Mode:    {MODE} ({', '.join(sorted(modes)) if modes else 'passive'})")
    print(f"[*] Screen:  {RESOLUTION}")
    if RANDOM_SEED is not None:
        print(f"[*] Seed:    {RANDOM_SEED}")
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
