# Contributing to Ossuary

Thanks for your interest. Contributions are welcome under the terms of this
repository's licence.

## Licence of contributions

By opening a pull request you agree that your contribution is provided under the
project's licence, **MIT OR Apache-2.0**, with no additional terms.

## Before you start

- For anything larger than a small fix, please open an issue first so the scope
  can be agreed. This project is maintained in spare time and keeps scope small
  on purpose.
- Keep changes original. Do not copy code, art, glyph tables or assets from
  another project; see `docs/inspiration.md` for how inspiration is handled.

## Building and testing

The exact commands live in `docs/development.md`. In short:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --workspace -- -D warnings
cargo test --workspace
```

Logic belongs in `crates/ossuary-core` with a headless test; `apps/ossuary` is
only painting and input. New user-visible behaviour should come with a test.

CI is authoritative: a change is only “done” when the `check` workflow passes on
GitHub Actions.

## Pull requests

- Open the PR against `main` and fill in the template, including the Rust QA
  section.
- Keep the change small and focused.
- Do not include real personal data or secrets anywhere — including tests and
  fixtures. Use obviously synthetic values (`dev@example.com`, `Jane Doe`).

## Reporting bugs and ideas

Use the issue forms under `.github/ISSUE_TEMPLATE/`. For security issues, follow
`SECURITY.md`.
