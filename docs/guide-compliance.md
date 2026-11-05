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
| CI: fmt, clippy `-D warnings`, test, release build, artefact (§K.6) | Met. `.github/workflows/ci.yml`. |
| Release workflow from day one (§4) | Met. `.github/workflows/release.yml`. |
| PR template (§8.3) + Rust QA (§M.3) | Met. `.github/PULL_REQUEST_TEMPLATE.md`. |
| Personal-data & secret scans (§II.29) | Met as blocking steps in CI; scripts under `scripts/`. |
| No heavy local builds (§4, §K.11.8) | Met. All build/test ran in CI via `ci.sh`. |
| Token ledger (§12.1) | Met. `docs/tokens.json`; release workflow records each tag. |

## Adaptations (recorded, not assumed)

- **GitHub operations via `gh`, not a GitHub MCP server.** The server is not in
  this environment's tool catalog, so repository creation, PRs, checks and
  releases are driven by the authenticated `gh` CLI. The effect §D.1 asks for is
  achieved.
- **Secret scanning is a dependency-free in-repo grep**, not `gitleaks`. It runs
  as a required step of the `check` job. `gitleaks` remains a recommended deeper
  local tool.
- **`cargo-deny` is configured (`deny.toml`) but not yet a blocking CI step** for
  v0.1.0; licence hygiene is tracked in `ATTRIBUTION.md` and the SBOM is emitted
  in CI.
