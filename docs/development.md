# Development

## For everyone reading this repository

CI is authoritative. Every pull request must pass the `check` workflow
(format, clippy with no warnings, headless tests, a release build and the secret
and personal-data scans) before it can merge.

## Building and testing locally (for contributors)

You do **not** need any of this to *use* Ossuary — download a release binary
instead (see the README). These commands are for people changing the code:

```sh
cargo fmt --all -- --check      # formatting
cargo clippy --all-targets --workspace -- -D warnings
cargo test --workspace          # headless: no display, no network
cargo build --release -p ossuary
cargo run -p ossuary            # run the terminal app
```

Tests need no terminal or display; the simulation lives entirely in
`ossuary-core` (guide §K.13).

## Why CI is authoritative here

The maintainer's machine is deliberately constrained, so **all** heavy work
runs in GitHub Actions and none of it is run locally on that machine
(guide §4). This is a property of the maintainer's setup, not a restriction on
contributors: the commands above are exactly what CI runs.

## Layout

See `docs/architecture.md`. In short: put logic in `crates/ossuary-core` with a
headless test; keep `apps/ossuary` to painting and input.
