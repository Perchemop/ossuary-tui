#!/usr/bin/env bash
# Release-asset scan (guide II.24.3 / II.29.6): fails when a built artefact
# carries a maintainer home path or a real-looking email address. Build paths
# under CI users (/home/runner, /Users/runner, C:\Users\runner[admin]) and
# obviously synthetic values are acceptable.
#
# Classification of expected hits (guide II.29.2):
#   * Home paths: only the CI build user (/home/runner, ...) may appear; it is
#     a build machine, not a person.
#   * Emails: a small allowlist of TLS root-certificate contact addresses is
#     expected, because webpki-roots embeds them in any statically linked
#     binary. These are certificate-authority metadata, i.e. third-party
#     upstream content, not a person connected to this project. Any *other*
#     address is a finding and fails the scan.
#
# Usage: bash scripts/scan-release-assets.sh <extracted-asset-dir>
set -euo pipefail

TARGET="${1:?usage: scan-release-assets.sh <extracted-asset-dir>}"

HOME_RE='/home/[A-Za-z0-9_.-]+|/Users/[A-Za-z0-9_.-]+|C:\\Users\\[A-Za-z0-9_.-]+'
HOME_ALLOW='(/home/runner|/Users/runner|C:\\Users\\runner|C:\\Users\\runneradmin)$'
EMAIL_RE='[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}'
EMAIL_ALLOW='@(example\.(com|org|net)|users\.noreply\.github\.com|noreply\.github\.com)$'
# Third-party TLS root-certificate contact addresses (webpki-roots/rustls).
THIRD_PARTY_EMAIL_ALLOW='(appro@openssl\.org|info@e-szigno\.hu)$'

# Search inside zips without extracting them, plus loose files in place.
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

while IFS= read -r -d '' archive; do
    case "$archive" in
        *.zip)
            if command -v unzip >/dev/null 2>&1; then
                unzip -qq -o "$archive" -d "$tmp" >/dev/null 2>&1 || true
            fi
            ;;
    esac
done < <(find "$TARGET" -type f -print0)

hits=""
if [ -d "$tmp" ] && [ -n "$(ls -A "$tmp" 2>/dev/null || true)" ]; then
    hits="$(
        grep -rhnaoE "$HOME_RE|$EMAIL_RE" "$tmp" 2>/dev/null \
            | grep -vE "$HOME_ALLOW|$EMAIL_ALLOW|$THIRD_PARTY_EMAIL_ALLOW" || true
    )"
fi

if [ -n "${hits//[[:space:]]/}" ]; then
    echo "ERROR: release asset(s) contain a non-CI home path or real email:" >&2
    echo "$hits" >&2
    exit 1
fi

echo "release-asset scan: clean ($TARGET)"
