# Attribution

Ossuary is licensed **MIT OR Apache-2.0** (see `LICENSE-MIT` and
`LICENSE-APACHE`). This file lists third-party components so the licence picture
is complete (guide §2.2, §II.19.3).

## Third-party Rust crates (direct dependencies)

| Crate | Version | Licence | Purpose |
|-------|---------|---------|---------|
| `serde` | 1 | MIT OR Apache-2.0 | Serialisation of the theme and payloads |
| `serde_json` | 1 | MIT OR Apache-2.0 | JSON theme files and API payloads |
| `thiserror` | 2 | MIT OR Apache-2.0 | Typed error enums in the core crate |
| `tempfile` (dev) | 3 | MIT OR Apache-2.0 | Isolated temp directories in workflow tests |
| `uuid` | 1 | Apache-2.0 OR MIT | Unique crash-log file names |
| `ureq` | 3 | MIT OR Apache-2.0 | Synchronous HTTP for the optional update check / crash submit |
| `crossterm` | 0.28 | MIT | Terminal raw mode, colours, cursor and events |

Notable transitive dependencies pulled in by `ureq`'s default TLS stack:

| Crate | Licence | Purpose |
|-------|---------|---------|
| `rustls` / `rustls-webpki` | Apache-2.0 OR ISC | TLS for the update check |
| `ring` | ISC AND MIT AND OpenSSL (permissive) | Cryptography for TLS |
| `webpki-roots` | MPL-2.0 | Bundled TLS root certificates |
| `icu_*` / `zerovec` / `writeable` | Unicode-3.0 | IDNA/URL handling |
| `memchr` | MIT OR Unlicense | Fast byte search |
| `adler2` | 0BSD OR MIT OR Apache-2.0 | Compression checksum |

The full transitive dependency list, with licences, is emitted as an SBOM
(`cargo tree`) by CI and can be regenerated with `cargo tree --workspace`. A
`deny.toml` allows only permissive licences (plus MPL-2.0 and the bundled font
licences) and bans strong copyleft.

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
