#!/usr/bin/env python3
"""Compatibility launcher for the Rust freeze hunter."""

import os
import sys


def main() -> None:
    os.execvp("cargo", ["cargo", "run", "-p", "xtask", "--", "freeze-hunter", *sys.argv[1:]])


if __name__ == "__main__":
    main()
