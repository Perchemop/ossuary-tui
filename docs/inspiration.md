# Inspiration (clean-room)

Capability-level inspiration only, from **public** sources. This file records
what other programs *do* so Ossuary can decide what to build — never their code,
art, glyph tables or exact look. Ossuary's own art was typed by hand and its
code was written from scratch (guide §2.1–2.2, §II.27.2).

Every claim below is an original one-line summary plus a source URL. No third
party's text, screenshots or assets are stored in this repository.

## Terminal rain / matrix screensavers

| Observable behaviour | Source (public) |
|----------------------|-----------------|
| A full-screen animation of falling character columns: each column has a bright leading character and a fading trail behind it | https://github.com/abishekvashok/cmatrix |
| Configurable speed, colour, column density and asynchronous column updates via command-line flags and keys | https://github.com/abishekvashok/cmatrix |
| Runs in the terminal's alternate screen, restores the terminal on exit, and responds to a quit key | https://github.com/abishekvashok/cmatrix |
| Glyphs are chosen from a character set (a classic set is half-width katakana) | https://github.com/abishekvashok/cmatrix |

**What Ossuary takes:** the *idea* of falling columns with a bright head and a
fading trail, run on the alternate screen, with speed/density controls.

**What Ossuary deliberately does NOT take:** the katakana glyph table, the exact
colour scheme, the source code (cmatrix is GPL-licensed and Ossuary is MIT OR
Apache-2.0), or any command-line interface or string copied from it. Ossuary's
flags, keys, art and palette are its own.

## Punctuation-based, monochrome type art (style reference)

| Observable behaviour | Source (public) |
|----------------------|-----------------|
| Pictures drawn only from monospace punctuation/symbols, hand-typed in a text editor, with shading from character density | https://vagonparovoz.tumblr.com/ |
| A "flicker" is produced by swapping individual characters between frames | https://www.reddit.com/user/vagonparovoz/ |
| A black-and-white palette; complete pictures rather than only falling text | https://objkt.com/ (vagonparovoz artist description) |

**What Ossuary takes:** an *aesthetic direction* — sparse punctuation, a
monochrome black-and-white palette, character-swap flicker, and the notion that
whole pictures can emerge from typed characters.

**What Ossuary deliberately does NOT take:** any of that artist's actual ASCII
art. Every art piece shipped in `crates/ossuary-core/src/art.rs` was typed by
hand for Ossuary and is original. The style is not copyrightable; a specific
drawing is.

## Human gate

Per guide §2.3 / §8.4 these notes are original summaries plus URLs. Any feature
that depends on them is implemented only after the notes are accepted.
