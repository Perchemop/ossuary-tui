# Ossuary

**Ossuary** turns your terminal into a screen of falling characters that
periodically assemble into hand-typed ASCII art — a skull, a set of bones, a
cat, a little candle, a ghost — in crisp monochrome, with a soft glitchy
flicker. Think of it as a matrix-style screensaver, but instead of endless
katakana it types real pictures out of punctuation and then lets them dissolve
back into the rain.

## Why you might like it

- **Free and open source.** No cost, no account, no trial limits. The source is
  openly available under the MIT OR Apache-2.0 licence.
- **Completely offline.** Ossuary never needs the internet. The only optional
  network use is checking for a newer release, and only when you ask it to.
- **Runs anywhere.** It is a small terminal program, so it works on Linux, macOS
  and Windows, over SSH, and on a spare monitor.
- **Your own art.** Drop simple text files into a folder and they join the rain.

## Get started

1. **Download.** Open the [Releases](https://github.com/Perchemop/ossuary-tui/releases)
   page and download the file for your computer (Linux, macOS or Windows). The
   files are built automatically by GitHub Actions, so every release matches the
   published source code.
2. **Unzip.** Extract the archive to any folder you like.
3. **Run.** Open a terminal in that folder and start Ossuary:

   - **Linux / macOS:** `./ossuary`
   - **Windows:** `ossuary.exe`

   (You do not need to install anything else.)

Press `q` to quit, or `?` for the full list of keys.

## What you can do

- **Watch the rain.** Colourful-to-monochrome columns of characters fall down
  the screen with a bright head and a fading trail.
- **See the pictures.** Every so often the characters gather into a complete
  picture, hold it for a moment, then melt away. Press `n` to summon one now.
- **Change the mood.** Swap the character set (`g`), speed it up or slow it down
  (`+` / `-`), make it denser or sparser (`[` / `]`), change the flicker (`f`),
  and switch between monochrome and the classic green (`m`).
- **Save your look.** Press `s` and your settings are remembered next time.
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

- Everything happens on your computer. Ossuary works fully offline and nothing
  you do is ever uploaded.
- There is no account, no sign-in and no telemetry.
- If you press `u`, Ossuary reads public release information from GitHub to
  check for a newer version. If you are offline, it simply says so and keeps
  working.
- If Ossuary ever crashes, it writes a small, **scrubbed** log to its logs
  folder. Home paths and anything resembling a secret are removed before the
  log is written, and nothing is ever sent automatically.

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
| `s` | save settings |
| `u` | check for updates |
| `V` | release notes |
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
