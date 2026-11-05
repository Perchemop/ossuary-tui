#!/usr/bin/env bash
# Personal-data scan (guide II.29, owner directive 2026-10-27): fails when a
# real-looking personal value is committed. It is a required status check so a
# leak cannot merge.
#
# Usage:
#   bash scripts/scan-personal-data.sh              # scan git-tracked files
#   bash scripts/scan-personal-data.sh --path DIR   # scan a directory
set -euo pipefail

cd "$(dirname "$0")/.."

MODE="git"
SCAN_ROOT=""
if [ "${1:-}" = "--path" ]; then
    MODE="path"
    SCAN_ROOT="${2:?usage: scan-personal-data.sh [--path <dir>]}"
fi

HOME_RE='/home/[A-Za-z0-9_.-]+|/Users/[A-Za-z0-9_.-]+|C:\\Users\\[A-Za-z0-9_.-]+'
HOME_ALLOW='(/home/runner|/Users/runner|C:\\Users\\runner|C:\\Users\\runneradmin)$'
EMAIL_RE='[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}'
EMAIL_ALLOW='@(example\.(com|org|net)|users\.noreply\.github\.com|noreply\.github\.com)$'

list_files() {
    if [ "$MODE" = path ]; then
        find "$SCAN_ROOT" -type f -print0
    else
        git ls-files -z -- ':!scripts/scan-personal-data.sh' ':!scripts/scan-release-assets.sh'
    fi
}

home_hits="$(
    list_files | while IFS= read -r -d '' f; do
        [ -f "$f" ] || continue
        grep -HnaoE "$HOME_RE" "$f" 2>/dev/null || true
    done | grep -vE "$HOME_ALLOW" || true
)"

email_hits="$(
    list_files | while IFS= read -r -d '' f; do
        [ -f "$f" ] || continue
        grep -HnaoE "$EMAIL_RE" "$f" 2>/dev/null || true
    done | grep -vEi "$EMAIL_ALLOW" || true
)"

fail=0
if [ -n "${home_hits//[[:space:]]/}" ]; then
    echo "ERROR: file(s) contain a non-CI absolute home path:" >&2
    echo "$home_hits" >&2
    fail=1
fi
if [ -n "${email_hits//[[:space:]]/}" ]; then
    echo "ERROR: file(s) contain a non-synthetic email address:" >&2
    echo "$email_hits" >&2
    fail=1
fi
if [ "$fail" -ne 0 ]; then
    echo "Classify each hit (guide II.29.2): synthetic/CI, or a finding to fix." >&2
    exit 1
fi

if [ "$MODE" = path ]; then
    echo "personal-data scan: clean ($SCAN_ROOT)"
else
    echo "personal-data scan: clean (no non-CI home path, no real email)"
fi
