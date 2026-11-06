# Roadmap

Progressive, honest milestones. Readiness percentages move only from verified
evidence (guide §9, §M.1) — a named test or a CI run. Percentages are of the v1
scope in `docs/scope.md`.

## Status

- **v0.1.0 — first functional test build**: shipped (see `CHANGELOG.md`).
- **v1 scope complete.** Every in-scope capability is implemented and verified;
  the headless tests and a release build run on Linux, macOS and Windows in CI,
  and a pseudo-terminal smoke test proves the app runs and draws.

## Milestones

| Milestone | Contents | State |
|-----------|----------|-------|
| v0.1.0 | Rain, glyph sets, original art + lifecycle, live controls, theme persistence, user art, version/notes overlay, crash crate, CI | done |
| v0.2.0 | `Cargo.lock` committed; `cargo-deny` a blocking gate; ten original art pieces | done |
| v0.3.0 | **TUI-only distribution**: no published binaries, fully offline (update check removed) | done |
| v0.4.0 | Art-rotation modes (sequential/random/single); per-column colour drift; optional art titles; a first-run welcome overlay | done |
| v0.5.0 | SVG preview renderer + README preview images generated in CI | done |
| v1.0.0 | §9 quality gates and the §2.5 legal checklist clean; cross-OS CI matrix + pty smoke test; v1 scope complete | done |

> **Distribution (owner directive, 2026-11-06):** Ossuary ships as **source
> only** — there are no downloadable release binaries and no desktop installers.
> It is a terminal program you build or `cargo install` yourself.

## Feature breadth (v1 scope)

| Area | Breadth | Ready for work |
|------|---------|----------------|
| Simulation & rendering | ~90% | yes |
| Glyph sets | ~90% | yes |
| Art model & lifecycle | ~85% | yes |
| Controls & persistence | ~85% | yes |
| Offline / no network | 100% | yes |

Percentages are estimates of the v1 scope only, and are capped by what CI has
actually verified.
