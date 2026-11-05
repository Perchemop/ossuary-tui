# Project Brief — Ossuary

Discovery answers per guide §0. This is the source of truth for scope
questions; `docs/scope.md` holds the in/out list and
`docs/assumptions-and-questions.md` holds everything unconfirmed.

Facts / Assumptions / Decisions are marked `[Fact]`, `[Assumption]`,
`[Decision]` throughout.

## A. Product identity

1. **Purpose (one sentence)**: A free terminal "screen" that rains monochrome
   ASCII characters which periodically assemble into hand-typed art pieces
   (skulls, bones, and friends) with a sparse, glitchy, typewriter feel.
   `[Decision]`
2. **Primary user**: Anyone who likes terminal toys and screensavers — the
   `cmatrix`-and-`sl` crowd, retro-computing fans, and people who leave a
   terminal running on a spare monitor. `[Assumption — inferred; not separately
   confirmed]`
3. **Problem solved**: The classic matrix screensaver look is decades old and
   purely decorative katakana. Ossuary offers an original, offline,
   dependency-light alternative with a different visual language (punctuation
   type-art and whole pictures) and it runs anywhere a terminal runs.
   `[Decision]`
4. **Suite membership**: Yes — part of the same free-Rust-app suite as Rubric
   and Lintel, and it consumes the shared `crash-report` and `update-check`
   crates. `[Decision]`

## B. Inspiration & scope

5. **Feature inspiration**: `docs/inspiration.md` — public behaviour only.
   `[Decision]`
6. **Most important capabilities (v0.1.0)**: a falling-character rain; a set of
   original art pieces that are typed into existence and then dissolve;
   on-the-fly control of glyph set, speed, density and flicker; and saving the
   settings to disk so a chosen look persists. `[Decision]`
7. **Out of scope for v1**: a GUI window, network features beyond the optional
   update check, sound, and multiplayer/shared screens. Full list in
   `docs/scope.md`. `[Decision]`

## C. Technical preferences

8. **Language/framework**: Pure Rust. Terminal shell uses `crossterm`; the
   simulation is a headless, fully-tested core crate. `[Decision]`
9. **Primary local platform**: Linux (the maintainer's machine).
   `[Fact]`
10. **Target platforms after local success**: Linux, macOS and Windows — all
    three are practical for a `crossterm` app and are built in the release
    workflow. `[Decision]`
11. **Offline**: Yes — the app works with no network. The only optional network
    use is the user-initiated update check, which fails soft. `[Decision]`
12. **Dependencies/licences**: Free OSS only; permissive licences preferred,
    strong copyleft prohibited without approval. Full list: `ATTRIBUTION.md`.
    `[Decision]`

## D. Quality & process

13. **Polish**: A clean, coherent, original look; the "keyboard is the UI"
    convention of good TUI software. `[Decision]`
14. **Constrained machine**: The local host is memory-constrained; **all** heavy
    work (format, lint, test, release build) runs in GitHub Actions.
    `[Fact — AGENTS.md "CI-first"]`
15. **Crash + update**: Shared `crash-report` (panic hook to a log file) and
    `update-check` (GitHub Releases API, soft-fail) crates. `[Decision]`

## E. Name

16. **Name**: **Ossuary** — invented for this project during discovery and
    locked; see `docs/product-name.md`. `[Decision]`
