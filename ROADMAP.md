# Roadmap

Progressive, honest milestones. Readiness percentages move only from verified
evidence (guide §9, §M.1) — a named test or a CI run. Percentages are of the v1
scope in `docs/scope.md`.

## Status

- **v0.1.0 — first functional test build**: shipped (see `CHANGELOG.md`).
- **v1 scope complete.** Every in-scope capability is implemented and verified;
  the headless tests and a release build run on Linux, macOS and Windows in CI,
  and a pseudo-terminal smoke test proves the app runs and draws.
- **v1.1.0 — new art pieces**: skeleton, coffin, tombstone added; cat and
  hourglass refined with tone-mapped shading | done |
- **v1.5.0 — full art pack redraw** (2026-11-07): every picture piece redrawn
  with organic contours and tonal ramps; full-scene graveyard and sanctum added;
  guide §12.2 parallel staff simulation adopted | done |
- **v1.7.0 — scope-named art completed** (2026-11-08): carriage, gem and goose
  added, completing the set named in `docs/scope.md` | done |
- **v1.8.0 — simulation polish layer** (2026-11-08): new workspace crate
  `crates/ossuary-sim-polish` provides composable enhancements without modifying
  `ossuary-core`:
  - `column_smoothing::ColumnSmoothing` — one-pole EMA for per-column head
    jitter reduction, unit-tested.
  - `art_reveal_ease` — `ease_out_cubic` and `ease_in_out_quad` pure easing
    curves for reveal timing, unit-tested.
  - Crate is pure, headless-tested, and follows parallel-agent build hygiene
    (new module only, dedicated `CARGO_TARGET_DIR`, no edits to existing engine).

## Milestones

| Milestone | Contents | State |
|-----------|----------|-------|
| v0.1.0 | Rain, glyph sets, original art + lifecycle, live controls, theme persistence, user art, version/notes overlay, crash crate, CI | done |
| v0.2.0 | `Cargo.lock` committed; `cargo-deny` a blocking gate; ten original art pieces | done |
| v0.3.0 | **TUI-only distribution**: no published binaries, fully offline (update check removed) | done |
| v0.4.0 | Art-rotation modes (sequential/random/single); per-column colour drift; optional art titles; a first-run welcome overlay | done |
| v0.5.0 | SVG preview renderer + README preview images generated in CI | done |
| v1.0.0 | §9 quality gates and the §2.5 legal checklist clean; cross-OS CI matrix + pty smoke test; v1 scope complete | done |
| v1.1.0 | Three new original art pieces (skeleton, coffin, tombstone); cat and hourglass refined | done |
| v1.2.0 | Three new original art pieces (rose, bell, book) | done |
| v1.3.0 | New original art piece (cross) | done |
| v1.4.0 | New original art piece (urn) | done |
| v1.4.1 | Restored the full built-in art pack after a botched edit dropped pieces | done |
| v1.5.0 | Full art pack redraw: organic contours, tonal ramps, full-scene graveyard + sanctum; guide §12.2 parallel-staff adoption | done |
| v1.6.0 | Token ledger instrumented with real session data; readable token metrics in the release overlay; CI ledger validation gate | done |
| v1.7.0 | Scope-named art completed: carriage, gem, goose added (28 picture pieces) | done |
| v1.8.0 | Simulation polish layer — `crates/ossuary-sim-polish` with `ColumnSmoothing` and `art_reveal_ease`, no engine edits, build-hygiene compliant | done |

> **Distribution (owner directive, 2026-11-06):** Ossuary ships as **source
> only** — there are no downloadable release binaries and no desktop installers.
> It is a terminal program you build or `cargo install` yourself.

## Feature breadth (v1 scope)

| Area | Breadth | Ready for work |
|------|---------|----------------|
| Simulation & rendering | ~95% (v1.9.0 target ~98%) | yes |
| Glyph sets | ~90% (v1.9.0 target ~90%, regression-only) | yes |
| Art model & lifecycle | ~88% (v1.9.0 target ~91%) | yes |
| Controls & persistence | ~88% (v1.9.0 target ~92%) | yes |
| Offline / no network | 100% | yes |

Percentages are estimates of the v1 scope only, and are capped by what CI has
actually verified.
