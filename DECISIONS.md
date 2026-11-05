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

## 2026-11-06 — D-006: cargo-deny is a blocking gate

**Decision:** `cargo-deny check` runs as a required step of the CI `check` job.
**Why:** Guide §II.20.4 asks for a dependency/licence gate. Making it blocking
means a copyleft dependency, a yanked crate or an unexpected licence fails the
pull request rather than being noticed at release time.

## 2026-11-06 — D-005: Dual licence MIT OR Apache-2.0

**Decision:** Ossuary is licensed MIT OR Apache-2.0.
**Why:** The suite default (guide §2.2), confirmed deliberately here per
§II.26.6.
