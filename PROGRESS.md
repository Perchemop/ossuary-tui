# Progress Log

Append-only log of work done, with the evidence for each claim.

## 2026-11-06 — Session 1: discovery, foundation and v0.1.0

**Goal:** take Ossuary from an empty repository to a green v0.1.0 test build.

### Done

- **Discovery** recorded in `docs/project-brief.md`, `docs/scope.md`,
  `docs/inspiration.md`, `docs/product-name.md`,
  `docs/assumptions-and-questions.md`. Four decisions were confirmed with the
  client (terminal app; name Ossuary; interactive art toy; original art + user
  packs) — see `docs/assumptions-and-questions.md`.
- **Foundation**: Cargo workspace, `rust-toolchain.toml`, shared `crash-report`
  and `update-check` crates, `.gitignore`, `deny.toml`.
- **Core** (`crates/ossuary-core`): deterministic RNG, glyph sets, art model,
  config/persistence, the rain engine — with unit tests in every module and a
  headless workflow suite in `tests/workflow.rs`.
- **Shell** (`apps/ossuary`): `crossterm` event loop, painter, key handling,
  crash hook, update check, version + release-notes overlay.
- **CI and release** workflows, PR template, issue forms, scan scripts.
- **Docs**: brand, architecture, capability inventory, parity, development,
  guide-compliance, QC checklist.

### Evidence

- CI `check` run: _pending first push_ — this line is updated with the run id
  once the first green run completes.
- Verification is done in GitHub Actions only (guide §4); no `cargo` ran
  locally.

### Next

- Commit the CI-generated `Cargo.lock` (uploaded as the `ossuary-cargo-lock`
  artefact).
- Tag and publish `v0.1.0`.
- Begin v0.2.0 items from `ROADMAP.md`.

### Process note

One friction item logged in `docs/process-feedback.md` (bootstrapping
`Cargo.lock` without a local toolchain).
