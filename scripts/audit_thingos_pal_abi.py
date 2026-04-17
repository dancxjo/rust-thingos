#!/usr/bin/env python3
"""Small ABI drift audit for ThingOS std PAL syscall/ABI mirrors."""

from __future__ import annotations

import re
import sys
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent


def read(path: str) -> str:
    return (REPO_ROOT / path).read_text(encoding="utf-8")


def check() -> list[str]:
    errors: list[str] = []

    numbers = read("library/std/src/sys/thingos_syscall_numbers.rs")
    if 'include!("../../../../thingos/abi/src/numbers.rs");' not in numbers:
        errors.append(
            "thingos syscall numbers are no longer sourced from thingos/abi/src/numbers.rs"
        )

    for rel in (
        "library/std/src/sys/pal/thingos/os.rs",
        "library/std/src/sys/pal/thingos/process.rs",
        "library/std/src/sys/pal/thingos/time.rs",
    ):
        text = read(rel)
        if re.search(r"(?m)^\\s*const\\s+SYS_[A-Z0-9_]+\\s*:", text):
            errors.append(f"{rel} defines local SYS_* constants (should import shared numbers)")

    process_text = read("library/std/src/sys/pal/thingos/process.rs")
    for name in ("SpawnProcessExReq", "SpawnProcessExResp", "FdRemap"):
        if re.search(rf"(?m)^\\s*struct\\s+{name}\\b", process_text):
            errors.append(f"process.rs locally defines {name} (should use thingos::abi)")

    time_text = read("library/std/src/sys/pal/thingos/time.rs")
    if re.search(r"(?m)^\\s*struct\\s+TimeSpec\\b", time_text):
        errors.append("time.rs locally defines TimeSpec (should use thingos::abi)")

    return errors


def main() -> int:
    errors = check()
    if errors:
        print("ThingOS PAL ABI audit: FAILED")
        for item in errors:
            print(f" - {item}")
        return 1

    print("ThingOS PAL ABI audit: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
