# Requirements Traceability

Each requirement id maps to its implementation and to the test or CI evidence
that proves it "done" (guide §II.24.2).

| ID | Requirement | Implementation | Evidence | State |
|----|-------------|----------------|----------|-------|
| R1 | Discovery docs | `docs/*.md` | present in tree | DONE |
| R2 | Foundation + CI | `Cargo.toml`, `.github/workflows/*` | CI run 42101993829 — success | DONE |
| R3 | Deterministic rain engine | `ossuary-core/src/engine.rs` | `engine::tests::*`, `workflow_rain_*`, `workflow_same_seed_*` | DONE |
| R4 | Glyph sets | `ossuary-core/src/glyphs.rs` | `glyphs::tests::*` | DONE |
| R5 | Art model + lifecycle | `ossuary-core/src/art.rs`, `engine.rs` | `art::tests::*`, `workflow_art_piece_*` | DONE |
| R6 | Config + persistence | `ossuary-core/src/config.rs` | `workflow_theme_save_load_roundtrip`, `config::tests::*` | DONE |
| R7 | User art packs | `ossuary-core/src/art.rs` | `workflow_user_art_pack_*` | DONE |
| R8 | Terminal shell | `apps/ossuary/src/*.rs` | builds in CI; hand-tested | DONE |
| R9 | Version + notes overlay | `apps/ossuary/src/app.rs` | `include_str!` of CHANGELOG/tokens | DONE |
| R10 | Crash log (no network) | `crates/crash-report` | shared-crate unit tests | DONE |
| R11 | Legal + docs baseline | `LICENSE*`, `ATTRIBUTION.md`, `README.md` | present in tree | DONE |
| R12 | CI green + v0.1.0 | `.github/workflows/ci.yml` | CI run 42101993829 — success | DONE |
| R13 | Lockfile + cargo-deny | `Cargo.lock`, `deny.toml` | `Cargo.lock` committed (from CI); `cargo-deny` configured but not a blocking gate yet | PARTIAL |
| R14 | macOS/Windows hands-on | release artefacts | manual | OPEN |
| R15 | More art pieces | `art.rs` | tests | OPEN |
| R16 | In-app update install | `apps/ossuary/src/app.rs` | manual | OPEN |
| R17 | Screenshots + first-run | README, shell | manual | OPEN |
| R18 | Finished-product gates | all | §9 + §2.5 checklist | OPEN |

Ids in this file and in `IMPLEMENTATION_PLAN.md` are kept in step; there are no
orphans.
