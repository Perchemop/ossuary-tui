# ASCII Art Expertise Development Log

## Goal
Develop genuine expertise in terminal ASCII art (small-scale, monochrome, hand-typed)
sufficient to redesign Ossuary's art pack so it reads as scene-style solid/grayscale
work rather than lineart-with-shading-tacked-on.

## Method
Research → Analyze reference works → Prototype → Evaluate → Iterate → Document

## Phase 1: Research (in progress)
- [x] Density ramps (`@%#*+=-:. `, Bourke 70-step)
- [x] Shape-based character selection (upper/lower cell density)
- [x] Anti-aliasing (`.`, `,`, `'`, `"`, `:`, `;`, `-`, `_`, `~`, `` ` ``)
- [x] Solid/grayscale tradition (filler chars $ # 8 & @ %)
- [x] Dithering theory (Bayer, Floyd-Steinberg, blue noise)
- [x] Structure-based vs tone-based ASCII
- [ ] Character-by-character analysis of reference pieces
- [ ] vagonparovoz technique analysis

## Phase 2: Analysis tools
- [ ] Ink density calculator per character
- [ ] Tone distribution analyzer
- [ ] Edge quality checker (anti-aliasing detection)
- [ ] Aspect ratio corrector (2:1 char cell)

## Phase 3: Art pack iteration
- [ ] Skull (signature piece - must read clearly at small scale)
- [ ] Crossbones
- [ ] Additional pieces (hourglass, key, chalice, etc.)

## Phase 4: Documentation
- [ ] Write up technique notes
- [ ] Record decisions per piece
- [ ] Create before/after comparisons

## Constraints
- Terminal monospace, ~20-25 columns wide (Ossuary house style)
- Pure ASCII (7-bit, printable characters 32-126)
- Monochrome (brightness ramp, not color)
- Hand-typed aesthetic (not converter output)
