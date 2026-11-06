# Attribution

Ossuary is licensed **MIT OR Apache-2.0** (see `LICENSE-MIT` and
`LICENSE-APACHE`). This file lists third-party components so the licence picture
is complete (guide §2.2, §II.19.3).

## Third-party Rust crates (direct dependencies)

| Crate | Version | Licence | Purpose |
|-------|---------|---------|---------|
| `serde` | 1 | MIT OR Apache-2.0 | Serialisation of the theme and of crash reports |
| `serde_json` | 1 | MIT OR Apache-2.0 | JSON theme files and crash-log payloads |
| `thiserror` | 2 | MIT OR Apache-2.0 | Typed error enums in the core crate |
| `tempfile` (dev) | 3 | MIT OR Apache-2.0 | Isolated temp directories in workflow tests |
| `uuid` | 1 | Apache-2.0 OR MIT | Unique crash-log file names |
| `crossterm` | 0.28 | MIT | Terminal raw mode, colours, cursor and events |

Ossuary has **no network dependencies**: the vendored `crash-report` crate only
writes a local log, so no TLS/HTTP stack is linked in.

The full transitive dependency list, with licences, is emitted as an SBOM
(`cargo tree`) by CI and can be regenerated with `cargo tree --workspace`. A
`deny.toml` allows only permissive licences and bans strong copyleft.

## Fonts and assets

- No font is embedded or shipped: Ossuary uses the terminal's own monospace
  font.
- `assets/logo.svg` and `assets/icons/ossuary.svg` are original to this project.

## Art

Every ASCII art piece shipped in `crates/ossuary-core/src/art.rs` was typed by
hand for Ossuary and is original work of this project.

## AI tooling used during the build

The application was authored with AI coding assistance (OpenCode free models and
the NVIDIA free tier), under the same clean-room review bar as any other
contribution: all committed material is original or permissively licensed, and
no third party's code or art was copied.
