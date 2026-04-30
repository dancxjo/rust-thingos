#!/usr/bin/env python3
import os
import signal
import subprocess
import sys
import time
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parent))
import freeze_hunter


class FreezeHunterTests(unittest.TestCase):
    def test_parse_modes_accepts_aliases_and_combinations(self):
        self.assertEqual(freeze_hunter.parse_modes("idle"), set())
        self.assertEqual(freeze_hunter.parse_modes("circle"), {"mouse-circle"})
        self.assertEqual(freeze_hunter.parse_modes("alt_tab"), {"alt-tab"})
        self.assertEqual(freeze_hunter.parse_modes("circle,alt-tab"), {"mouse-circle", "alt-tab"})
        self.assertEqual(freeze_hunter.parse_modes("active"), {"mouse-circle", "alt-tab"})
        self.assertEqual(freeze_hunter.parse_modes("random"), {"random-input"})
        self.assertEqual(freeze_hunter.parse_modes("stress"), {"mouse-circle", "alt-tab", "random-input"})

    def test_compose_qemu_flags_adds_display_and_optional_qmp(self):
        flags = freeze_hunter.compose_qemu_flags("-m 2G -smp 4", "/tmp/qmp.sock")
        self.assertIn("-display none", flags)
        self.assertIn("-qmp unix:/tmp/qmp.sock,server=on,wait=off", flags)

        flags = freeze_hunter.compose_qemu_flags("-m 2G -display gtk -qmp tcp:127.0.0.1:4444")
        self.assertEqual(flags, "-m 2G -display gtk -qmp tcp:127.0.0.1:4444")

    def test_run_session_active_mode_enables_qmp_input(self):
        script = "import sys, time; sys.stdout.write('booting ' * 40); sys.stdout.flush(); time.sleep(10)"

        with TemporaryDirectory() as tmp:
            captured_env = {}
            with mock.patch.object(freeze_hunter, "LOG_DIR", str(Path(tmp) / "logs")), \
                 mock.patch.object(freeze_hunter, "TIMEOUT", 0.25), \
                 mock.patch.object(freeze_hunter, "ARCH", "x86_64"), \
                 mock.patch.object(freeze_hunter, "LOG_LEVEL", "5"), \
                 mock.patch.object(freeze_hunter, "MODE", "active"), \
                 mock.patch.object(freeze_hunter, "QMP_CONNECT_TIMEOUT", 0.05):

                original_popen = subprocess.Popen

                def fake_popen(cmd, *args, **kwargs):
                    captured_env.update(kwargs.get("env") or {})
                    return original_popen(
                        [sys.executable, "-c", script],
                        stdout=kwargs["stdout"],
                        stderr=kwargs["stderr"],
                        stdin=kwargs["stdin"],
                        env=captured_env,
                        preexec_fn=os.setsid,
                    )

                with mock.patch.object(subprocess, "Popen", side_effect=fake_popen):
                    self.assertTrue(freeze_hunter.run_session(8))

                self.assertIn("-qmp unix:", captured_env["QEMUFLAGS"])
                self.assertIn("qmp.sock,server=on,wait=off", captured_env["QEMUFLAGS"])

    def test_random_input_emits_qmp_events(self):
        driver = freeze_hunter.QmpInputDriver("/tmp/qmp.sock", {"random-input"})
        calls = []

        with mock.patch.object(driver, "_execute", side_effect=lambda qmp, command: calls.append(command)), \
             mock.patch.object(freeze_hunter.random, "choice", side_effect=["key", "ret"]):
            driver._send_random_input(object())

        self.assertEqual(len(calls), 2)
        self.assertEqual(calls[0]["execute"], "input-send-event")
        self.assertEqual(calls[0]["arguments"]["events"][0]["type"], "key")
        self.assertEqual(calls[0]["arguments"]["events"][0]["data"]["key"]["data"], "ret")
        self.assertTrue(calls[0]["arguments"]["events"][0]["data"]["down"])
        self.assertFalse(calls[1]["arguments"]["events"][0]["data"]["down"])

    def test_run_session_times_out_after_partial_line_output(self):
        script = (
            "import sys, time; "
            "sys.stdout.write('serial prompt without newline ' * 10); "
            "sys.stdout.flush(); "
            "time.sleep(10)"
        )

        with TemporaryDirectory() as tmp:
            with mock.patch.object(freeze_hunter, "LOG_DIR", str(Path(tmp) / "logs")), \
                 mock.patch.object(freeze_hunter, "TIMEOUT", 0.25), \
                 mock.patch.object(freeze_hunter, "ARCH", "x86_64"), \
                 mock.patch.object(freeze_hunter, "LOG_LEVEL", "5"), \
                 mock.patch.object(freeze_hunter, "MODE", "idle"):

                original_popen = subprocess.Popen

                def fake_popen(cmd, *args, **kwargs):
                    env = os.environ.copy()
                    env.update(kwargs.get("env") or {})
                    return original_popen(
                        [sys.executable, "-c", script],
                        stdout=kwargs["stdout"],
                        stderr=kwargs["stderr"],
                        stdin=kwargs["stdin"],
                        env=env,
                        preexec_fn=os.setsid,
                    )

                start = time.time()
                with mock.patch.object(subprocess, "Popen", side_effect=fake_popen):
                    self.assertTrue(freeze_hunter.run_session(7))
                self.assertLess(time.time() - start, 2.0)

                logs = list((Path(tmp) / "logs").glob("run_0007_*.log"))
                self.assertEqual(len(logs), 1)
                content = logs[0].read_text()
                self.assertIn("serial prompt without newline", content)
                self.assertIn("KILLED DUE TO SILENCE", content)


if __name__ == "__main__":
    unittest.main()
