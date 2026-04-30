#!/usr/bin/env python3
import subprocess
import time
import sys
import os
import signal
import select
from datetime import datetime

# Configuration
TIMEOUT = float(os.environ.get("HUNTER_TIMEOUT", "5.0"))
ARCH = os.environ.get("KARCH", "x86_64")
LOG_DIR = "freeze_logs"
LOG_LEVEL = os.environ.get("HUNTER_LOGLEVEL", "5") # 5 = Trace

def build_once():
    print("[*] Performing initial build...")
    subprocess.run(["just", "iso", ARCH], check=True)

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
    os.makedirs(LOG_DIR, exist_ok=True)
    timestamp = datetime.now().strftime("%H%M%S")
    log_path = os.path.join(LOG_DIR, f"run_{session_id:04d}_{timestamp}.log")
    
    # Use just run. It handles building and standard environment setup (audio, etc).
    # We force -display none to ensure it stays in the terminal for log capture.
    cmd = [
        "just", "run", ARCH, "--loglevel", LOG_LEVEL
    ]
    
    print(f"[*] Session {session_id:04d} | Log: {log_path}", end="\r")
    
    env = os.environ.copy()
    env["QEMUFLAGS"] = env.get("QEMUFLAGS", "-m 2G -smp 4") + " -display none"
    
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
            process.stdout.close()
            
    runtime = time.time() - start_time
    # If it died very quickly with almost no output, it might be a build error or config issue
    if byte_count < 200 and runtime < 1.0:
        print(f"\n[!] Session {session_id} failed to produce output. Check build or QEMU logs.")
        return False
        
    return True

def main():
    print(f"=== ThingOS Freeze Hunter ===")
    print(f"[*] Arch:    {ARCH}")
    print(f"[*] Timeout: {TIMEOUT}s")
    print(f"[*] Level:   {LOG_LEVEL}")
    print(f"[*] Logs:    {LOG_DIR}/")
    print("[*] Ctrl+C to stop.")
    
    try:
        build_once()
        session_id = 0
        while True:
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
