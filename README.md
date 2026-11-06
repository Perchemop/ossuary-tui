# Ossuary

**Ossuary** turns your terminal into a screen of falling characters that
periodically assemble into hand-typed ASCII art — a skull, a set of bones, a
cat, a little candle, a ghost — in crisp monochrome, with a soft glitchy
flicker. Think of a matrix-style screensaver, but instead of endless katakana it
types real pictures out of punctuation and then lets them dissolve back into the
rain.

It is a terminal program (a TUI): you run it *in* your terminal, not as a
desktop window.

## Why you might like it

- **Free and open source.** No cost, no account, no trial limits. The source is
  openly available under the MIT OR Apache-2.0 licence.
- **Completely offline.** Ossuary makes no network requests at all. There is no
  telemetry and there is no update check.
- **Small and native.** A single terminal binary with a tiny dependency
  footprint. It runs anywhere a terminal runs, including over SSH.
- **Your own art.** Drop simple text files into a folder and they join the rain.

## Get started

Ossuary is a terminal application, so there is no installer and nothing to
download — you run it from your terminal. Build it once with Rust:

```sh
# install straight from the repository (needs the Rust toolchain)
cargo install --git https://github.com/Perchemop/ossuary-tui ossuary

# then, from any terminal, just:
ossuary
```

Or, from a clone of the repository, run it without installing:

```sh
cargo run -p ossuary
```

Press `q` to quit, or `?` for the full list of keys.

## What you can do

- **Watch the rain.** Columns of characters fall down the screen with a bright
  head and a fading trail.
- **See the pictures.** Every so often the characters gather into a complete
  picture, hold it for a moment, then melt away. Press `n` to summon one now.
- **Change the mood.** Swap the character set (`g`), speed it up or slow it down
  (`+` / `-`), make it denser or sparser (`[` / `]`), change the flicker (`f`),
  and switch between monochrome and the classic green (`m`).
- **Save your look.** Press `s` and your settings are remembered next time.
- **Vary the art.** Choose how pictures are picked (`r`: sequential, random, or
  one favourite on repeat), add a little depth with colour drift (`d`), and
  label the current piece with its name (`t`).
- **Add your own art.** Save any multi-line text drawing as a `.txt` file in the
  Ossuary art folder (see below) and it joins the rotation.

## Where your settings live

Ossuary keeps one small settings file and an art folder in your personal
configuration directory:

- **Linux:** `~/.config/ossuary/` (`theme.json` and an `art/` folder)
- **macOS:** `~/.config/ossuary/`
- **Windows:** `%APPDATA%\ossuary\`

Your own `.txt` art files go in the `art/` folder inside that directory.

## Your privacy

- Everything happens on your computer, and Ossuary never talks to the network —
  there is no update check and no telemetry.
- There is no account and no sign-in.
- If Ossuary ever crashes, it writes a small, **scrubbed** log to its `logs`
  folder. Home paths and anything resembling a secret are removed before the log
  is written, and nothing is ever sent anywhere.

## Keys at a glance

| Key | Action |
|-----|--------|
| `space` | pause / resume |
| `g` | next character set |
| `n` | summon an art piece |
| `a` | toggle automatic art |
| `+` / `-` | faster / slower |
| `[` / `]` | sparser / denser |
| `f` | cycle flicker |
| `m` | monochrome / green |
| `r` | next art rotation mode |
| `d` | toggle colour drift |
| `t` | toggle art titles |
| `s` | save settings |
| `V` | changelog |
| `?` | help |
| `q` | quit |

## Helping out

Bug reports and ideas are welcome through the [Issues](https://github.com/Perchemop/ossuary-tui/issues)
page. If you want to change the code, see `CONTRIBUTING.md` and
`docs/development.md`.

## Licence

Ossuary is free software, available under either the
[MIT](LICENSE-MIT) or the [Apache-2.0](LICENSE-APACHE) licence, at your choice.
Third-party components are listed in `ATTRIBUTION.md`.
