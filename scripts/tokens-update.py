#!/usr/bin/env python3
"""Token ledger tooling for guide §12.1.

Ossuary is built by OpenCode agent sessions, so "token metrics" are the tokens
consumed by those sessions. OpenCode records them per assistant message in a
local SQLite database (~/.local/share/opencode/opencode.db, WAL mode). This
script aggregates that data into the committed ledger `docs/tokens.json` and
validates the ledger in CI.

Counting rule (documented, applied consistently):
  tokens = input + output + reasoning   (cache *reads* are excluded: they are
  context re-reads, not generation; cache writes are recorded as 0 by the
  providers in use).

Subcommands:
  update   Recompute totals from the local OpenCode database and rewrite the
           ledger. `--since` sets the start (ISO date, UTC) of the current
           release window.
  validate Check the ledger's schema and that release_version matches the
           crate manifest. Used as a CI gate; exits non-zero on failure.

The ledger snapshot is taken when `update` runs (typically at release time), so
release_tokens is the usage up to that moment, not a live figure.
"""
from __future__ import annotations

import argparse
import datetime
import json
import re
import sqlite3
import sys
import tempfile
from contextlib import contextmanager
from pathlib import Path

LEDGER_FIELDS = ("total_tokens_since_start", "release_tokens", "release_version")


@contextmanager
def open_db_snapshot(db_path: Path):
    """Snapshot db + WAL via SQLite's backup API into a temp dir.

    The live database is WAL-mode, so copying files can race with writes; the
    online backup API gives a consistent copy instead. Usage:

        with open_db_snapshot(path) as db: ...
    """
    db_path = db_path.expanduser()
    with tempfile.TemporaryDirectory(prefix="ossuary-tokens-") as tmp:
        dest_path = Path(tmp) / db_path.name
        source = sqlite3.connect(f"{db_path.as_uri()}?mode=ro", uri=True)
        try:
            dest = sqlite3.connect(dest_path)
            try:
                source.backup(dest)
            finally:
                dest.close()
        finally:
            source.close()
        yield sqlite3.connect(dest_path)


def find_sessions(db: sqlite3.Connection, directory: str, title: str) -> list[str]:
    """Sessions for this project (by directory or title) plus their children."""
    ids: set[str] = set()
    rows = db.execute(
        "select id, directory, title from session_v2"
    ).fetchall()
    for sid, sess_dir, sess_title in rows:
        if (directory and sess_dir == directory) or (title and sess_title == title):
            ids.add(sid)
    # one level of children (subagent sessions)
    for sid in list(ids):
        for (child,) in db.execute(
            "select id from session_v2 where parent_id=?", (sid,)
        ):
            ids.add(child)
    return sorted(ids)


def aggregate(
    db: sqlite3.Connection, sessions: list[str], since_ms: float
) -> tuple[int, int]:
    """Return (total_tokens, tokens_since) over assistant messages.

    tokens_since counts messages created at or after `since_ms` (epoch ms, UTC)
    -- the release window, passed as an explicit `--since` date so it matches
    the slice the release is cut from.
    """
    total = 0
    since = 0
    for sid in sessions:
        for (data,) in db.execute(
            "select data from session_message where session_id=? and type='assistant'",
            (sid,),
        ):
            try:
                msg = json.loads(data)
            except json.JSONDecodeError:
                continue
            tokens = msg.get("tokens") or {}
            count = (
                (tokens.get("input") or 0)
                + (tokens.get("output") or 0)
                + (tokens.get("reasoning") or 0)
            )
            total += count
            created = (msg.get("time") or {}).get("created", 0)
            if created and created >= since_ms:
                since += count
    return total, since


def read_manifest_version(manifest: Path) -> str:
    text = manifest.read_text(encoding="utf-8")
    match = re.search(r'^version\s*=\s*"([^"]+)"', text, re.MULTILINE)
    if not match:
        raise SystemExit(f"no version found in {manifest}")
    return match.group(1)


def version_key(version: str) -> tuple[int, ...]:
    """Numeric sort key so `1.10.0` sorts after `1.5.0`."""
    try:
        return tuple(int(part) for part in version.split("."))
    except ValueError:
        return (0,)


def cmd_update(args: argparse.Namespace) -> int:
    with open_db_snapshot(Path(args.db)) as db:
        sessions = find_sessions(db, args.directory, args.title)
        if not sessions:
            raise SystemExit("no matching sessions found; refusing to write zeros")
        if args.since:
            since_ms = (
                datetime.datetime.fromisoformat(args.since)
                .replace(tzinfo=datetime.UTC)
                .timestamp()
                * 1000
            )
        else:
            since_ms = (
                datetime.datetime.now(datetime.UTC)
                .replace(hour=0, minute=0, second=0, microsecond=0)
                .timestamp()
                * 1000
            )
        total, since = aggregate(db, sessions, since_ms)
    ledger_path = Path(args.file)
    ledger = json.loads(ledger_path.read_text(encoding="utf-8"))
    version = args.release_version or ledger.get("release_version")
    if not version:
        raise SystemExit("no release_version in ledger; pass --release-version")
    ledger["total_tokens_since_start"] = total
    ledger["release_tokens"] = since
    ledger["release_version"] = version
    releases = {
        r["version"]: r.get("tokens", 0)
        for r in ledger.get("releases", [])
        if isinstance(r, dict) and "version" in r
    }
    releases[version] = since
    ledger["releases"] = [
        {"version": v, "tokens": releases[v]} for v in sorted(releases, key=version_key)
    ]
    ledger_path.write_text(json.dumps(ledger) + "\n", encoding="utf-8")
    print(
        f"ledger updated: total={total} release={since} "
        f"version={version} sessions={len(sessions)}"
    )
    return 0


def cmd_validate(args: argparse.Namespace) -> int:
    ledger_path = Path(args.file)
    try:
        ledger = json.loads(ledger_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        print(f"FAIL {ledger_path}: invalid JSON ({exc})")
        return 1
    for field in LEDGER_FIELDS:
        if field not in ledger:
            print(f"FAIL {ledger_path}: missing canonical field {field}")
            return 1
    if not isinstance(ledger["total_tokens_since_start"], int):
        print("FAIL total_tokens_since_start must be an integer")
        return 1
    if not isinstance(ledger["release_tokens"], int):
        print("FAIL release_tokens must be an integer")
        return 1
    if not isinstance(ledger["release_version"], str):
        print("FAIL release_version must be a string")
        return 1
    manifest_version = read_manifest_version(Path(args.manifest))
    if ledger["release_version"] != manifest_version:
        print(
            f"FAIL release_version {ledger['release_version']!r} != "
            f"manifest version {manifest_version!r}"
        )
        return 1
    for entry in ledger.get("releases", []):
        if "version" not in entry or "tokens" not in entry:
            print(f"FAIL malformed releases entry: {entry}")
            return 1
    print(
        f"ledger OK: total={ledger['total_tokens_since_start']} "
        f"release={ledger['release_tokens']} version={ledger['release_version']}"
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    p_update = sub.add_parser("update", help="recompute the ledger from the local DB")
    p_update.add_argument("--db", default="~/.local/share/opencode/opencode.db")
    p_update.add_argument("--directory", default=None)
    p_update.add_argument("--title", default=None)
    p_update.add_argument(
        "--since",
        default=None,
        help="ISO date (UTC) starting the current release window",
    )
    p_update.add_argument("--file", default="docs/tokens.json")
    p_update.add_argument("--release-version", default=None)
    p_update.set_defaults(func=cmd_update)

    p_validate = sub.add_parser("validate", help="validate the ledger (CI gate)")
    p_validate.add_argument("--file", default="docs/tokens.json")
    p_validate.add_argument("--manifest", default="apps/ossuary/Cargo.toml")
    p_validate.set_defaults(func=cmd_validate)

    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
