#!/usr/bin/env python3
"""Record a release's token count in docs/tokens.json (guide §12.1).

Usage: python3 scripts/update-token-ledger.py <version> <release-tokens>

Idempotent: re-running for a version already recorded only corrects that
version's figure, never double-counting.
"""
from __future__ import annotations

import json
import pathlib
import sys

LEDGER = pathlib.Path(__file__).resolve().parent.parent / "docs" / "tokens.json"


def main() -> int:
    if len(sys.argv) < 3:
        print("usage: update-token-ledger.py <version> <release-tokens>", file=sys.stderr)
        return 2
    version = sys.argv[1]
    release_tokens = int(sys.argv[2])

    ledger = {"total_tokens_since_start": 0, "releases": []}
    if LEDGER.exists():
        try:
            ledger = json.loads(LEDGER.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            pass

    releases = [r for r in ledger.get("releases", []) if r.get("version") != version]
    releases.append({"version": version, "tokens": release_tokens})
    releases.sort(key=lambda r: r.get("version", ""))
    ledger["releases"] = releases
    ledger["total_tokens_since_start"] = sum(int(r.get("tokens", 0)) for r in releases)

    LEDGER.parent.mkdir(parents=True, exist_ok=True)
    LEDGER.write_text(json.dumps(ledger, indent=2) + "\n", encoding="utf-8")
    print(f"ledger updated: {version} = {release_tokens:,} tokens")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
