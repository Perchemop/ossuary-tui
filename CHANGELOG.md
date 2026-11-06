# Changelog

All notable changes to Ossuary are recorded here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

- Cumulative since project start: see `docs/tokens.json`.
- This release: recorded by the release workflow.
