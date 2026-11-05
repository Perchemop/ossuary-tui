# Implementation Plan

Phased plan (guide §8.1). Requirement IDs trace to
`REQUIREMENTS_TRACEABILITY.md`.

## Phase 1 — Discovery  ✔

- R1 Brief, scope, inspiration, name, assumptions → `docs/*.md`.

## Phase 2 — Foundation  ✔

- R2 Cargo workspace, toolchain pin, shared crates, CI + release workflows,
  PR template, issue forms, scan scripts.

## Phase 3 — v0.1.0 hard gate  ✔

- R3 Deterministic rain engine (core), unit-tested.
- R4 Glyph sets (core), unit-tested.
- R5 Original art model + lifecycle (core), workflow-tested.
- R6 Config + persistence (core), workflow-tested.
- R7 User art packs (core), tested.
- R8 Terminal shell: loop, painter, keys (`crossterm`).
- R9 Version display + release-notes overlay.
- R10 Crash hook + update check via shared crates.
- R11 README, CHANGELOG, LICENSE, ATTRIBUTION.
- R12 CI green; release `v0.1.0`.

## Phase 4 — Post-v0.1.0  ◻

- R13 Commit `Cargo.lock`; make `cargo-deny` blocking.
- R14 Hands-on verification on macOS and Windows.
- R15 Additional original art pieces.
- R16 In-app one-click update.

## Phase 5 — Polish & multi-platform  ◻

- R17 README screenshots; first-run overlay; broader platform QA.

## Phase 6 — Finished product  ◻

- R18 §9 gates and §2.5 legal checklist clean.
