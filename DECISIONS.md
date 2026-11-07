# Decisions

Append-only log of dated decisions. Each entry states the decision and why.

## 2026-11-06 — D-001: Language and UI toolkit

**Decision:** Pure Rust; the terminal shell uses `crossterm` and the simulation
is a headless `ossuary-core` crate.
**Why:** The app is a terminal toy, so a TUI toolkit is the honest choice. The
core/shell split is required for headless CI testing (guide §K.3, §K.13).

## 2026-11-06 — D-002: Product name

**Decision:** The product is named **Ossuary**.
**Why:** The guide requires an original name (§0, §II.18). See
`docs/product-name.md`.

## 2026-11-06 — D-003: Deterministic RNG

**Decision:** Hand-rolled xorshift64* in `ossuary-core` rather than a `rand`
dependency.
**Why:** It is tiny, dependency-free and reproducible across platforms, which is
exactly what deterministic frame tests need (guide §M.5).

## 2026-11-06 — D-004: Shared suite crates vendored

**Decision:** `crash-report` and `update-check` are vendored into this
repository (same shape as the other suite apps).
**Why:** Consistency with the suite (guide §7); the guide §12.1 note about a
submodule is not yet adopted here.
**Superseded in part (2026-11-06, D-007):** `update-check` was later removed
when Ossuary became fully offline and source-only.

## 2026-11-06 — D-007: TUI-only, source-only distribution, no network

**Decision:** Ossuary is distributed as **source only** — no published binaries
and no desktop installers — and the application makes **no network requests**
(the in-app update check and the crash auto-submission path were removed).
**Why:** Owner directive, 2026-11-06: "I only want this app available as a tui,
not desktop releases." This narrows `docs/scope.md` and overrides the guide's
release-artefact default; it is recorded here rather than as a silent omission.

## 2026-11-06 — D-006: cargo-deny is a blocking gate

**Decision:** `cargo-deny check` runs as a required step of the CI `check` job.
**Why:** Guide §II.20.4 asks for a dependency/licence gate. Making it blocking
means a copyleft dependency, a yanked crate or an unexpected licence fails the
pull request rather than being noticed at release time.

## 2026-11-06 — D-005: Dual licence MIT OR Apache-2.0

**Decision:** Ossuary is licensed MIT OR Apache-2.0.
**Why:** The suite default (guide §2.2), confirmed deliberately here per
§II.26.6.

## 2026-11-07 — D-008: Suite standards §12.1 reconciliation

**Decision:** Record explicit adoption/ deviations from guide §12.1 suite standards (token ledger, update-check submodule, signed releases, version UI, README screenshots).

**Why:** `guide.md` now consolidates the former `addendum.md` (appendices A–O). The suite standards added in §12.1 require a CI-managed token ledger with fields `total_tokens_since_start`, `release_tokens`, `release_version`; a shared `crates/update-check` submodule; signed release artefacts with `SHA256SUMS.txt`; in-app version display with release-notes popup; and refreshed README screenshots per major release.

For Ossuary:
- Token ledger exists (`docs/tokens.json`) with the canonical fields; metrics remain uninstrumented and are logged in `docs/process-feedback.md`.
- `update-check` submodule is deliberately omitted; D-007 makes the app fully offline and source-only.
- Signed releases and in-app auto-update are not applicable (source-only distribution).
- Version UI is adapted to the TUI: version is drawn bottom-right; `V` opens the release-history overlay with changelog and token metrics.
- README previews are generated in CI and will be refreshed at v1.0 and each major release.

Recorded in `docs/guide-compliance.md` (Recent guide changes, 2026-11-07).
