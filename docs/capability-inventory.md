# Capability Inventory

Single source of truth for what Ossuary is designed to do (guide §II.27.3). A
capability not listed here cannot be claimed done or reported missing.

| ID | Capability | Acceptance criteria | Status |
|----|------------|---------------------|--------|
| CAP-001 | Falling-character rain | Engine produces a grid of cells with bright heads and fading trails; deterministic per seed | implemented |
| CAP-002 | Built-in glyph sets | ≥3 named sets; cycling walks all and returns | implemented |
| CAP-003 | Original art pieces | ≥4 non-empty built-in pieces, all hand-typed and original | implemented |
| CAP-004 | Art lifecycle | A summonable piece is typed, held, then dissolved back to rain | implemented |
| CAP-005 | Live keyboard control | glyph set, speed, density, flicker, palette, pause, auto-art, summon | implemented |
| CAP-006 | Theme persistence | Settings save to JSON and reload identically | implemented |
| CAP-007 | User art packs | `*.txt` files in the art dir load as pieces; missing dir is not an error | implemented |
| CAP-008 | Version display | Version shown subtly in a corner | implemented |
| CAP-009 | Changelog overlay | Key opens the changelog + effort metrics in-app | implemented |
| CAP-010 | Crash capture | Panic hook writes a scrubbed local log | implemented |
| CAP-019 | Cross-OS CI | Headless tests + release build run on Linux, macOS and Windows | implemented |
| CAP-020 | Runtime smoke test | CI runs the binary under a pty and asserts it draws | implemented |
| CAP-018 | Preview renderer | The core renders a frame to a standalone SVG (used for README images) | implemented |
| CAP-014 | Art rotation modes | `r` cycles sequential → random → single-piece | implemented |
| CAP-015 | Colour drift | `d` varies each column's brightness band within the ramp | implemented |
| CAP-016 | Art titles | `t` labels the current piece with its name | implemented |
| CAP-017 | First-run welcome | A fresh install opens the key help automatically | implemented |
| CAP-011 | In-app update check | Not wanted — Ossuary is fully offline (owner directive) | out of scope |
| CAP-012 | Published binaries | Not wanted — source distribution only (owner directive) | out of scope |
| CAP-013 | Graphical window | Not wanted | out of scope |

Implementation evidence (test names, CI runs) is tracked in `docs/parity.md`.
