#!/usr/bin/env python3
"""Print the built-in art pack so it can be reviewed without building.

Parses the `== name` headers straight out of the art pack data file.

Usage: python3 scripts/show-art.py [substring-filter]
"""
from __future__ import annotations

import pathlib
import sys

SRC = pathlib.Path(__file__).resolve().parent.parent / "assets/art/pieces.txt"


def main() -> int:
    text = SRC.read_text(encoding="utf-8")
    blocks = text.split("== ")
    needle = sys.argv[1] if len(sys.argv) > 1 else ""
    for block in blocks:
        block = block.strip()
        if not block:
            continue
        lines = block.split("\n")
        name = lines[0].strip()
        if needle and needle not in name:
            continue
        art = lines[1:]
        while art and not art[-1].strip():
            art.pop()
        if not art:
            continue
        width = max(len(line) for line in art)
        ink = sum(1 for line in art for ch in line if ch != " ")
        print(
            f"─ {name}  ({len(art)} rows x {width} cols, {ink} ink) "
            + "─" * max(0, 40 - len(name))
        )
        for line in art:
            print("│" + line + " " * (width - len(line)) + "│")
        print("└" + "─" * (width + 2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
