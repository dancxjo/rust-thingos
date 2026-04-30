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
                 mock.patch.object(freeze_hunter, "LOG_LEVEL", "5"):

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
