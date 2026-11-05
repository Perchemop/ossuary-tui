#!/usr/bin/env python3
"""Print the CHANGELOG section for a version, for GitHub release notes.

Usage: python3 scripts/release-notes.py <version-without-leading-v>

Falls back to an honest one-liner when the version has no section yet.
"""
from __future__ import annotations

import pathlib
import sys

CHANGELOG = pathlib.Path(__file__).resolve().parent.parent / "CHANGELOG.md"


def section(version: str) -> str:
    if not CHANGELOG.exists():
        return ""
    lines = CHANGELOG.read_text(encoding="utf-8").splitlines()
    collected: list[str] = []
    capturing = False
    for line in lines:
        if line.startswith("## "):
            if capturing:
                break
            if version in line:
                capturing = True
                collected.append(line[3:].strip())
                continue
        elif capturing:
            collected.append(line)
    return "\n".join(collected).strip()


def main() -> int:
    version = sys.argv[1] if len(sys.argv) > 1 else ""
    body = section(version) if version else ""
    if not body:
        body = f"Automated release build for Ossuary {version or 'unknown'}."
    print(body)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
