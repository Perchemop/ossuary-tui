# Scope — Ossuary v1

Source of truth for "is this feature in the first major version?".

## In scope (v1)

| Area | What ships | Status target |
|------|------------|---------------|
| Rain simulation | Falling character columns with a per-column head and fading trail | v0.1.0 |
| Glyph sets | Several built-in character sets (punctuation, shade ramp, digits, letters, ASCII) | v0.1.0 |
| Art pieces | Original hand-typed pieces (skull, bones, cat, candle, goose, carriage, ghost, key, moon, gem) typed into existence, held, then dissolved | v0.1.0 / v0.2.0 |
| Live controls | Keyboard control of glyph set, speed, density, flicker, palette, pause, auto-art | v0.1.0 |
| Persistence | A JSON theme file (settings) saved and reloaded | v0.1.0 |
| User art packs | `.txt` pieces dropped into the art directory are loaded and shown | v0.1.0 |
| In-app version + changelog | Version in the corner; a changelog overlay readable from inside the app | v0.1.0 |
| Crash log | A panic hook writes a scrubbed, local crash log | v0.1.0 |

## Out of scope (non-goals) — v1

- **Published binaries of any kind.** Ossuary is a terminal program you build or
  `cargo install` yourself; there are no downloadable releases and no desktop
  installers (owner directive, 2026-11-06).
- **Any network feature.** No update check, no telemetry, no crash
  auto-submission. The app is fully offline.
- A graphical window (Ossuary is deliberately a terminal application).
- Audio.
- Plugin or scripting systems.
- Copying any other program's art, glyph tables or code (see `docs/inspiration.md`).

## MVP definition

The MVP = the v0.1.0 hard gate (guide §6.1): a runnable terminal program that
rains characters, types original art pieces, responds to the keyboard, and
persists a theme to disk — all verified by headless tests and CI.
