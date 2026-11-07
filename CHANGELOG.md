# Changelog

All notable changes to Ossuary are recorded here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.6.0] - 2026-11-07

### Added

- **Readable token metrics in the release overlay** (`V`): the ledger's three
  canonical fields are parsed from `docs/tokens.json` and drawn as
  `tokens since start: N | this release: N (vX.Y.Z)` instead of raw JSON; the
  raw-JSON rendering remains as the fallback.
- **Token ledger instrumentation** (closes the §12.1 friction item):
  `scripts/tokens-update.py` aggregates real per-message token counts from the
  build sessions into `docs/tokens.json` (input + output + reasoning; cache
  reads excluded), and CI validates the ledger schema and that
  `release_version` matches the crate manifest.

### Fixed

- Records corrected: the process-feedback entry about the uninstrumented
  ledger (previously referenced but missing) is now actually written, and the
  CHANGELOG/guide-compliance claim that "the release workflow records each
  tag" was removed — there is no release workflow (source-only, D-007).

### Token usage

- Cumulative since project start and this release window: see
  `docs/tokens.json`; updated by `scripts/tokens-update.py` at release time.

## [1.5.0] - 2026-11-07

### Changed

- **Complete art pack redraw** (22 original picture pieces): organic contours
  replace box frames, per-piece tonal ramps (bright fill, dim edges, true void
  eye sockets), anti-aliased edges, and two full-scene compositions
  (`graveyard` 39×15, `sanctum` 43×13). All pieces verified against
  `scripts/analyze_art.py` and the `tone()` ramp in `ossuary-core`.

### Process

- **Guide §12.2 parallel staff simulation adopted**: slices are run by a 6–8
  role agent team (persistent supervisor + parallel workers). Role → subagent
  mapping and the free-model capacity shrink plan recorded in
  `docs/current-model-map.md`; guide sync date 2026-11-07.

## [1.4.1] - 2026-11-06

### Fixed

- **Restored the full built-in art pack**: a botched edit had replaced the
  `candle` piece with a sparser duplicate and dropped the pieces between
  `candle` and the phrases (`moon`, `ghost`, `hourglass`, `pentagram`, `key`,
  `chalice`, `raven`, `rose`, `bell`, `book`, `skeleton`, `coffin`,
  `tombstone`, `cross`, `urn`). All 19 picture pieces and 3 phrases are back,
  every picture carries at least 30 ink cells, and there are no duplicate names.

## [1.4.0] - 2026-11-06

### Added

- **One new original art piece**: `urn` (funerary urn with tone-mapped shading)

## [1.3.0] - 2026-11-06

### Added

- **One new original art piece**: `cross` (cross with tone-mapped shading)

## [1.2.0] - 2026-11-06

### Added

- **Three new original art pieces**: `rose` (flower with bright petals), `bell` (church bell), `book` (open book)
- All pieces use tone-mapped glyphs with anti-aliased edges

## [1.1.0] - 2026-11-06

### Added

- **Three new original art pieces**: `skeleton` (full-body figure with carved void eyes), `coffin` (coffin box with cross pattern), `tombstone` (grave marker with RIP in bright glyphs)
- **Improved cat**: void eyes with shadow areas around the face
- **Improved hourglass**: mid-tone characters in transition zones for smoother shading

### Changed

- **Ossuary core and apps bumped to 1.1.0**

## [1.0.0] - 2026-11-06

First stable release. Ossuary is feature-complete for its v1 scope
(`docs/scope.md`).

### Added

- **Cross-platform CI matrix**: the headless tests and a release build now run
  on **Linux, macOS and Windows** on every change, so "works on more than one
  OS" is CI-proven rather than assumed.
- **Pseudo-terminal smoke test** (`scripts/pty-smoke.py`): CI starts the real
  binary under a pty and asserts it enters the alternate screen, hides the
  cursor and paints a screenful of glyphs.

### Changed

- `ossuary` / `ossuary-core` bumped to **1.0.0**.
- Both `check` and `matrix` are required status checks (`.ci-required.txt`).

### Known limitations

- Distributed as **source only** — no published binaries, by owner directive.
- Verified by headless tests plus a Linux pseudo-terminal smoke test; it has not
  been driven by hand across a wide range of real terminal emulators.
- Token metrics are recorded as 0 (no instrumentation).

## [0.5.0] - 2026-11-06

### Added

- **Preview renderer**: the headless core can render a frame to a standalone
  SVG (`ossuary_core::render_svg`), so the look can be captured and reviewed
  without a terminal. Fully unit-tested.
- **README preview images** under `docs/images/`, generated in CI from a fixed
  seed with `cargo run --example preview`.

### Changed

- `ossuary` and `ossuary-core` bumped to **0.5.0**.

## [0.4.0] - 2026-11-06

More art variety, plus first-run polish.

### Added

- **Art rotation modes** (`r`): cycle sequential → random → single-piece.
- **Colour drift** (`d`): each column's brightness is nudged within the
  monochrome ramp for a little extra depth.
- **Art titles** (`t`): label the piece currently on screen with its name.
- **First-run welcome**: a fresh install opens the key help automatically.

### Changed

- `ossuary` and `ossuary-core` bumped to **0.4.0**.

### Known limitations

- Token metrics are recorded as 0 (no instrumentation).

## [0.3.0] - 2026-11-06

Ossuary is now a self-contained, offline terminal application distributed as
**source only** — there are no published binaries.

### Changed

- **Distribution:** removed the release workflow and the four-platform binary
  artefacts. Install with `cargo install --git https://github.com/Perchemop/ossuary-tui ossuary`
  or run it from a clone.
- **Fully offline:** removed the in-app update check and the vendored
  `update-check` crate, and stripped the unused network-submit path from
  `crash-report`. No TLS/HTTP stack is linked into the binary any more.
- **Keys:** the `u` (update check) key is gone; `V` now opens the in-app
  changelog.
- README rewritten around running Ossuary in your terminal.

### Removed

- `.github/workflows/release.yml`, `crates/update-check`, and the release-only
  helper scripts.

### Known limitations

- Token metrics are recorded as 0 (no per-release instrumentation).

## [0.2.0] - 2026-11-06

Second release: a stricter supply-chain gate and a larger art pack.

### Added

- **Blocking `cargo-deny` gate** in CI. Licences, security advisories, duplicate
  bans and dependency sources are checked on every pull request, so a GPL
  dependency or a yanked crate can no longer merge unnoticed.
- **Four more original art pieces**: `ghost`, `key`, `moon` and `gem` — ten
  hand-typed pieces in total.

### Changed

- `ossuary` and `ossuary-core` bumped to **0.2.0**.
- `deny.toml` licence allow-list extended (`Unlicense`, `0BSD`, `OpenSSL`) to
  match the permissive licences actually present in the dependency tree.

### Known limitations

- macOS and Windows artefacts are produced by CI but have not yet been tried
  hands-on on those systems.
- Token metrics remain recorded as 0 (no per-release instrumentation yet).

## [0.1.0] - 2026-11-06

First functional test build. Ossuary is a terminal "screen" that rains
monochrome ASCII characters which periodically assemble into hand-typed art
pieces. This release establishes the working core and the path to release.

### Added

- **Falling-character rain**: per-column heads with fading trails, a
  deterministic simulation and a live resize path.
- **Glyph sets**: `bone`, `shade`, `digits`, `letters` and `ascii`, cyclable at
  runtime.
- **Original art pieces**: six hand-typed pieces — `skull`, `bones`, `cat`,
  `candle`, `goose` and `carriage` — typed into existence, held, then dissolved.
- **Live keyboard control**: pause, glyph set, summon, auto-art, speed, density,
  flicker, palette, save.
- **Theme persistence**: settings saved to `theme.json` and reloaded on start.
- **User art packs**: `*.txt` files in the art directory are loaded as pieces.
- **In-app version and release notes**: the version sits in the bottom-right
  corner; `V` opens the changelog and token metrics.
- **Crash capture**: a panic hook writes a scrubbed crash log.
- **Update check**: `u` queries GitHub Releases and fails soft when offline.
- **CI and release workflows**: format, clippy (no warnings), headless tests, a
  release build, artefact upload, secret and personal-data scans, and
  cross-platform release packaging.

### Known limitations

- Only the Linux binary is verified hands-on so far; macOS and Windows artefacts
  are produced by the release workflow and have not yet been tried on those
  systems.
- The update check has no in-app one-click installer yet; it reports that an
  update exists and where to get it.
- `cargo-deny` is configured but not yet a blocking CI step.

### Try it

Download the Linux artefact from this release, run `./ossuary`, and press `?` to
see the keys. Press `g`, `n` and `s` to change the glyphs, summon a picture and
save your settings.

### Token usage

- Cumulative since project start: 19,844,575 tokens; this release window
  (2026-11-07): 6,661,364 — see `docs/tokens.json`.
- Method: aggregated from the build sessions by `scripts/tokens-update.py`
  (input + output + reasoning; cache reads excluded). There is no release
  workflow — source-only distribution (D-007) — so the ledger is updated at
  release time and validated in CI.
