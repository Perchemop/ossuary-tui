# QC Checklist

Run before a release. Record the result and the CI run id below.

## Build & tests

- [ ] CI `check` run is green on the released commit (run id: ______)
- [ ] Release workflow produced artefacts for all four targets
- [ ] Release assets scanned clean for personal data
- [ ] `docs/tokens.json` records the release

## Terminal smoke test (primary OS, by hand)

- [ ] `ossuary` starts and fills the screen with falling characters
- [ ] An art piece is typed into existence, held and dissolved
- [ ] `space` pauses; `q` quits and the terminal is restored (cursor visible,
      no leftover raw mode, original screen back)
- [ ] `g` cycles glyph sets; `+`/`-`, `[`/`]`, `f`, `m` change the look live
- [ ] `s` saves the theme; relaunching restores it
- [ ] `?` opens help; `V` opens release notes; both close with `Esc`
- [ ] `u` checks for updates; offline it says so and keeps running
- [ ] Resizing the terminal reflows the rain without crashing

## Accessibility (guide §II.21)

- [ ] Every action is reachable from the keyboard (there is no mouse UI)
- [ ] Focus/position is never conveyed by colour alone (brightness bands)
- [ ] `--version` and `--help` print plain text
- [ ] Reduced-motion: users can pause with `space`; noted as the app's answer to
      motion sensitivity
- [ ] Screen-reader limitation recorded in `ACCESSIBILITY.md`

## Legal (§2.5)

- [ ] `LICENSE-MIT` and `LICENSE-APACHE` present
- [ ] `ATTRIBUTION.md` lists every third-party crate and licence
- [ ] No secrets, no real personal data in the tree, history or assets
- [ ] README claims match the built binaries

## QC run log

| Date | Commit | CI run | Verdict | Notes |
|------|--------|--------|---------|-------|
| 2026-11-06 | (first build) | pending | pending | v0.1.0 first functional test build |
