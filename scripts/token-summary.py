#!/usr/bin/env python3
"""Print a short token-usage summary for release notes (guide §12.1).

Usage: python3 scripts/token-summary.py <version> <release-tokens>
"""
from __future__ import annotations

import json
import pathlib
import sys

LEDGER = pathlib.Path(__file__).resolve().parent.parent / "docs" / "tokens.json"


def main() -> int:
    version = sys.argv[1] if len(sys.argv) > 1 else "unknown"
    release_tokens = int(sys.argv[2]) if len(sys.argv) > 2 else 0
    ledger = {"total_tokens_since_start": 0, "releases": []}
    if LEDGER.exists():
        try:
            ledger = json.loads(LEDGER.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            pass
    total = ledger.get("total_tokens_since_start", 0)
    print(f"- This release ({version}): {release_tokens:,} tokens")
    print(f"- Cumulative since project start: {total:,} tokens")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
