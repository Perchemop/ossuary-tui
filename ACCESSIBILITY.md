# Accessibility

Accessibility is treated as a correctness property, not an afterthought
(guide §II.21). This is a terminal application, so the relevant levers differ
from a GUI, and the honest limits are stated here.

## What works

- **Fully keyboard-driven.** Every action has a key; there is no mouse UI, so
  no action is mouse-only.
- **No colour-only signals.** State is carried by position and character, not
  by hue alone — the palette is a brightness ramp that also works in
  monochrome.
- **Plain-text CLI.** `ossuary --help` and `--version` print ordinary text that
  any screen reader in a terminal can read.
- **Motion control.** The animation is a big moving field, which some people
  find uncomfortable; `space` pauses it instantly, and `q` exits.
- **User's own font.** The app uses the terminal's configured font and does not
  embed or impose one, so the user's readability settings are respected.

## Known limitations

- **Animated content has no "reduced motion" auto-detection.** Terminals do not
  expose a standard reduced-motion signal, so Ossuary cannot detect the OS
  preference; pausing manually is the offered answer.
- **Screen-reader support is limited by the terminal.** Ossuary paints a grid of
  characters; whether that is announced usefully depends entirely on the user's
  terminal and screen reader. The app does not add a semantic layer over the
  grid. The interactive keys and their effects are documented in `README.md`
  and the in-app `?` help.
- **Contrast depends on the terminal.** Ossuary requests grey/white bands; the
  actual contrast is whatever the user's terminal theme provides.

## Reporting

Accessibility problems use the dedicated issue form
(`.github/ISSUE_TEMPLATE/accessibility.yml`) so they are triaged like any other
bug.
