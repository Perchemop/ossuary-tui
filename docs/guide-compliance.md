# Guide Compliance

Record of how this repository meets the build guide, including honest notes on
adaptations.

| Area | Status |
|------|--------|
| Clean-room (§2) | Met. Original code and original art; `docs/inspiration.md` is summaries + URLs only. |
| v0.1.0 hard gate (§6.1) | Met — see `CHANGELOG.md` and `PROGRESS.md`. |
| Core/shell split (§K.3, §K.13) | Met. `ossuary-core` is headless; `apps/ossuary` only paints and collects input. |
| Headless workflow test (§K.5, §M.2) | Met. `crates/ossuary-core/tests/workflow.rs`. |
| Toolchain pin (§K.11.6) | Met. `rust-toolchain.toml`. |
| tempfile dev-dependency (§K.11.7) | Met. In `crates/ossuary-core/Cargo.toml`. |
| CI: fmt, clippy `-D warnings`, test, `cargo-deny`, release build, pty smoke, cross-OS matrix (§K.6) | Met. `.github/workflows/ci.yml` (`check` + `matrix`). |
| Release workflow from day one (§4) | **Deliberately not adopted** — the owner directive is source-only distribution with no published binaries. |
| PR template (§8.3) + Rust QA (§M.3) | Met. `.github/PULL_REQUEST_TEMPLATE.md`. |
| Personal-data & secret scans (§II.29) | Met as blocking steps in CI; scripts under `scripts/`. |
| No heavy local builds (§4, §K.11.8) | Met. All build/test ran in CI via `ci.sh`. |
| Token ledger (§12.1) | Met. `docs/tokens.json`; release workflow records each tag. |

## Final gate review (as of v0.5.0, 2026-11-06)

**Guide §9 — quality gates**

| Gate | Status |
|------|--------|
| Every in-scope capability implemented and verified | Met for the v1 scope in `docs/scope.md`; parity per `docs/parity.md` |
| Main workflows end-to-end | Met (rain, art lifecycle, controls, save/reload) |
| Persistence and integrity tested | Met (`workflow_theme_save_load_roundtrip`) |
| No fake functionality | Met |
| CI green; multi-OS | CI green throughout; the headless tests and a release build run on Linux, macOS and Windows, and a pseudo-terminal smoke test proves the app runs and draws |
| Crash reporting works as designed | Met (local, scrubbed log; unit-tested) |
| ROADMAP and CHANGELOG accurate | Met |
| No secrets; licences listed; README user-friendly | Met (scans are required CI steps; ATTRIBUTION current) |
| §2.5 legal checklist clean | Met (see below) |

**§2.5 — legal checklist**

| Area | Status |
|------|--------|
| Copyright | Clean — original code and original art only |
| Clean-room | Clean — `docs/inspiration.md` is summaries + URLs; no RE material |
| Trademarks | Product name and mark are original; no vendor marks |
| Trade dress | Original visual language |
| Patents | No known patented technique implemented |
| Licence | MIT OR Apache-2.0; third-party list in ATTRIBUTION; `cargo-deny` blocks copyleft |
| Secrets | None in tree, history or artefacts; secret scan is required |
| Privacy | No network at all; crash log is local and scrubbed |
| Claims | README/roadmap make no affiliation or "drop-in replacement" claims |
| Research files | Summaries + URLs only |
| Export/crypto | No export-controlled code |

**Verified at v1.0.0:** headless tests on Linux, macOS and Windows; a release
build on each; a pseudo-terminal smoke test; and the full gate on every change.

**Not claimed:** hand-driving across a wide range of real terminal emulators.
Nothing in these docs asserts that.

## Adaptations (recorded, not assumed)

- **GitHub operations via `gh`, not a GitHub MCP server.** The server is not in
  this environment's tool catalog, so repository creation, PRs, checks and
  releases are driven by the authenticated `gh` CLI. The effect §D.1 asks for is
  achieved.
- **Secret scanning is a dependency-free in-repo grep**, not `gitleaks`. It runs
  as a required step of the `check` job. `gitleaks` remains a recommended deeper
  local tool.
- **`cargo-deny` is a blocking CI step** (v0.2.0 onward); licence hygiene is also
  tracked in `ATTRIBUTION.md`, and the SBOM is emitted in CI.
- **No release workflow / published binaries.** By owner directive (2026-11-06)
  Ossuary ships as source only; the app is installed with `cargo install` or run
  from a clone. This overrides the guide's release-artefact default.
- **No network features.** The update check and crash auto-submission were
  removed so the whole tree is offline.
