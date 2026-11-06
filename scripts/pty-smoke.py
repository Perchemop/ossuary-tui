#!/usr/bin/env python3
"""Run the TUI under a pseudo-terminal for a moment and assert it renders.

Headless and dependency-free. Exits non-zero unless the app enters the
alternate screen, hides the cursor and paints a screenful of glyphs, so "it
actually runs and draws" is proven in CI rather than asserted by hand.

Usage: python3 scripts/pty-smoke.py <path-to-ossuary-binary>
"""
from __future__ import annotations

import os
import pty
import re
import select
import signal
import sys
import time

# Matches CSI escape sequences (cursor moves, colours, private modes).
CSI = re.compile(rb"\x1b\[[0-9;?]*[A-Za-z]")
OSC = re.compile(rb"\x1b\][^\x07]*\x07")

ALT_SCREEN_ON = b"\x1b[?1049h"
CURSOR_HIDDEN = b"\x1b[?25l"
MIN_GLYPHS = 20


def run(binary: str, seconds: float = 3.0) -> tuple[bytes, int]:
    pid, fd = pty.fork()
    if pid == 0:  # child: become the app
        os.environ.setdefault("OSSUARY_CONFIG_DIR", "/tmp/ossuary-smoke")
        os.environ["TERM"] = "xterm-256color"
        os.execv(binary, [binary, "--seed", "1"])
        os._exit(127)  # only reached if execv fails

    os.set_blocking(fd, False)
    data = b""
    deadline = time.time() + seconds
    while time.time() < deadline:
        ready, _, _ = select.select([fd], [], [], 0.2)
        if ready:
            try:
                chunk = os.read(fd, 65536)
            except (OSError, BlockingIOError):
                continue
            if not chunk:
                break
            data += chunk
    os.kill(pid, signal.SIGKILL)
    os.waitpid(pid, 0)

    text = OSC.sub(b"", CSI.sub(b"", data)).decode("utf-8", "replace")
    glyphs = sum(1 for ch in text if ch not in " \r\n\t")
    return data, glyphs


def main() -> int:
    binary = sys.argv[1] if len(sys.argv) > 1 else "target/release/ossuary"
    data, glyphs = run(binary)

    problems = []
    if ALT_SCREEN_ON not in data:
        problems.append("did not enter the alternate screen")
    if CURSOR_HIDDEN not in data:
        problems.append("did not hide the cursor")
    if glyphs < MIN_GLYPHS:
        problems.append(f"painted too few glyphs ({glyphs})")

    if problems:
        print("pty smoke: FAILED: " + "; ".join(problems), file=sys.stderr)
        return 1
    print(f"pty smoke: ok ({len(data)} bytes, {glyphs} glyphs)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
