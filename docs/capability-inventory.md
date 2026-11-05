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
| CAP-009 | Release-notes overlay | Key opens changelog + token metrics in-app | implemented |
| CAP-010 | Crash capture | Panic hook writes a scrubbed log | implemented |
| CAP-011 | Update check | User-initiated GitHub Releases check, soft-fail offline | implemented |
| CAP-012 | Cross-platform builds | Linux, macOS, Windows release artefacts | partial — Linux verified in CI; others built by release workflow |
| CAP-013 | Graphical window | Not wanted | out of scope |

Implementation evidence (test names, CI runs) is tracked in `docs/parity.md`.
