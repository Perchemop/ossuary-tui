# Scope — Ossuary v1

Source of truth for "is this feature in the first major version?".

## In scope (v1)

| Area | What ships | Status target |
|------|------------|---------------|
| Rain simulation | Falling character columns with a per-column head and fading trail | v0.1.0 |
| Glyph sets | Several built-in character sets (punctuation, shade ramp, digits, letters, ASCII) | v0.1.0 |
| Art pieces | Original hand-typed pieces (skull, bones, cat, candle, goose, carriage) typed into existence, held, then dissolved | v0.1.0 |
| Live controls | Keyboard control of glyph set, speed, density, flicker, palette, pause, auto-art | v0.1.0 |
| Persistence | A JSON theme file (settings) saved and reloaded | v0.1.0 |
| User art packs | `.txt` pieces dropped into the art directory are loaded and shown | v0.1.0 |
| In-app version + notes | Version in the corner; a release-notes overlay with the changelog and token metrics | v0.1.0 |
| Crash + update | Panic hook logging; user-initiated update check | v0.1.0 |
| Packaging | Cross-platform release builds via CI | v0.2.0+ |

## Out of scope (non-goals) — v1

- A graphical window (Ossuary is deliberately a terminal application).
- Any network feature other than the optional, user-initiated update check.
- Audio.
- Plugin or scripting systems.
- Copying any other program's art, glyph tables or code (see `docs/inspiration.md`).

## MVP definition

The MVP = the v0.1.0 hard gate (guide §6.1): a runnable Linux binary that rains
characters, types original art pieces, responds to the keyboard, and persists a
theme to disk — all verified by headless tests and CI before release.
