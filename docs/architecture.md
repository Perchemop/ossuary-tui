# Architecture

Ossuary follows the suite's core/shell split (guide §K.3): the logic is a thick,
headless crate and the terminal is a thin shell.

```
apps/ossuary/          → terminal shell: raw mode, event loop, painter (crossterm)
crates/ossuary-core/   → simulation, glyph sets, art model, theme persistence
crates/crash-report/   → shared suite crate: panic hook, scrubbed capture
crates/update-check/   → shared suite crate: GitHub Releases, version compare
```

## Dependency direction

`apps/ossuary` depends on `ossuary-core`, `crash-report` and `update-check`.
`ossuary-core` depends on none of them: it has no terminal, no display and no
network code, so every workflow is testable headlessly (guide §K.13).

## Core (`ossuary-core`)

| Module | Responsibility |
|--------|----------------|
| `rng` | Deterministic xorshift64* generator; seeds make frames reproducible (guide §M.5) |
| `glyphs` | Named character sets and cycling |
| `art` | `ArtPiece` (rectangular, padded text) and `ArtPack` (built-in + user loads) |
| `config` | The persistable `Config` (settings/theme), clamping, and path helpers |
| `engine` | The rain (`Column`s) and the art lifecycle (`ArtState`), producing `Frame`s |

The engine has no clock: `step()` advances one frame, `frame()` returns the
result. The shell decides how often to call them.

## Data flow

1. The shell reads the terminal size and builds an `Engine`.
2. Each loop iteration: the shell polls for keys/resize, calls `engine.step()`
   (unless paused), then paints `engine.frame()`.
3. Key presses mutate a `Config`, which is pushed into the engine with
   `set_config`; `s` writes the `Config` to the theme file.

## Persistence

Settings live in a JSON theme file under the per-user config directory
(`~/.config/ossuary/theme.json` on Linux; `OSSUARY_CONFIG_DIR` overrides the
location). User art pieces are `*.txt` files under the adjacent `art/`
directory. Save/load is plain, atomic-enough `write`/`read` of a small file; a
missing or broken theme falls back to defaults rather than failing startup.
