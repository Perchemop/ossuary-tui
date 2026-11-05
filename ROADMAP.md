# Roadmap

Progressive, honest milestones. Readiness percentages move only from verified
evidence (guide §9, §M.1) — a named test or a CI run. Percentages are of the v1
scope in `docs/scope.md`.

## Status

- **v0.1.0 — first functional test build**: shipped (see `CHANGELOG.md`).
- **v1 scope readiness: ~75%** (verified: rain, glyphs, art lifecycle, controls,
  persistence, user art, version/notes, crash + update crates, CI. Unverified:
  hands-on macOS/Windows.)

## Milestones

| Milestone | Contents | State |
|-----------|----------|-------|
| v0.1.0 | Rain, glyph sets, original art + lifecycle, live controls, theme persistence, user art, version/notes overlay, crash + update crates, CI + release workflow | done |
| v0.2.0 | Commit the CI-generated `Cargo.lock`; make `cargo-deny` a blocking gate; verify macOS and Windows builds hands-on; a small set of additional original art pieces | planned |
| v0.3.0 | Multiple art-rotation modes; per-column colour drift within the monochrome ramp; optional title/labels for art pieces | planned |
| v0.4.0 | One-click in-app update from the latest release; a first-run "welcome" overlay | planned |
| v1.0.0 | All §9 quality gates and the §2.5 legal checklist clean; README screenshots; hands-on verification on all three platforms | planned |

## Feature breadth (v1 scope)

| Area | Breadth | Ready for work |
|------|---------|----------------|
| Simulation & rendering | ~90% | yes |
| Glyph sets | ~90% | yes |
| Art model & lifecycle | ~85% | yes |
| Controls & persistence | ~85% | yes |
| Platform packaging | ~40% | yes |
| In-app update install | ~30% | yes |

Percentages are estimates of the v1 scope only, and are capped by what CI has
actually verified.
