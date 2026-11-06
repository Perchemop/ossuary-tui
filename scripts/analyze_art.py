#!/usr/bin/env python3
"""ASCII art analysis toolkit for expert iteration.

Evaluates art pieces objectively using tone density, edge anti-aliasing,
ink distribution, and contour analysis. Designed for small-scale terminal
art (Ossuary house style: ~20-25 cols, pure ASCII, monochrome).
"""

from __future__ import annotations

import sys
from pathlib import Path

# Tone ramp: brightness level -> characters (darkest to lightest)
# Level 3 = brightest fill (lit bone), level 0 = void
TONE_MAP: dict[int, str] = {
    3: "$#8&@%WMB0QD",      # dense fill
    2: "SIXZOABCDEFGHJKLMNPQRSTUVWXYZ",  # mid tone
    1: ".,:'\"`^~-_~:;!|/\\()[]{}<>",   # dim edge/shadow
    0: " ",                          # void
}

# Reverse lookup: char -> tone level
CHAR_TONE: dict[str, int] = {}
for level, chars in TONE_MAP.items():
    for ch in chars:
        CHAR_TONE[ch] = level


def char_tone(ch: str) -> int:
    """Return the brightness level (0-3) of a character."""
    return CHAR_TONE.get(ch, 2)  # default to mid-tone for unknown


def tone_name(level: int) -> str:
    return {0: "void", 1: "dim", 2: "mid", 3: "bright"}[level]


def analyze_piece(name: str, lines: list[str]) -> None:
    """Print a full analysis of an ASCII art piece."""
    if not lines:
        print(f"{name}: empty")
        return

    width = max(len(line) for line in lines)
    height = len(lines)

    # Normalize lines to equal width
    padded = [line.ljust(width) for line in lines]

    # Count ink (non-space)
    ink_cells = 0
    tone_counts = {0: 0, 1: 0, 2: 0, 3: 0}
    edge_cells = 0

    for y, line in enumerate(padded):
        for x, ch in enumerate(line):
            if ch == " ":
                continue
            ink_cells += 1
            tone = char_tone(ch)
            tone_counts[tone] += 1

            # Edge detection: cell next to a space or boundary
            is_edge = False
            for dy, dx in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                ny, nx = y + dy, x + dx
                if ny < 0 or ny >= height or nx < 0 or nx >= width:
                    is_edge = True
                    break
                if padded[ny][nx] == " ":
                    is_edge = True
                    break
            if is_edge:
                edge_cells += 1

    print(f"═══ {name} ═══")
    print(f"  Dimensions: {width}x{height}")
    print(f"  Ink cells:  {ink_cells} ({ink_cells / (width * height) * 100:.1f}% density)")
    print(f"  Tone distribution:")
    for level in [3, 2, 1, 0]:
        count = tone_counts[level]
        pct = count / ink_cells * 100 if ink_cells else 0
        bar = "█" * int(pct / 5)
        print(f"    {tone_name(level):6s} (level {level}): {count:4d} ({pct:5.1f}%) {bar}")
    print(f"  Edge cells: {edge_cells} ({edge_cells / ink_cells * 100:.1f}% of ink)")
    print()

    # Tone balance check: good art should have a mix, not all bright
    bright_pct = tone_counts[3] / ink_cells * 100 if ink_cells else 0
    dim_pct = (tone_counts[0] + tone_counts[1]) / ink_cells * 100 if ink_cells else 0

    print(f"  Assessment:")
    if bright_pct > 80:
        print("    ⚠️  TOO BRIGHT: >80% ink is bright fill - lacks shading depth")
    elif bright_pct < 30:
        print("    ⚠️  TOO DARK: <30% ink is bright fill - may wash out")
    else:
        print("    ✓ Brightness balance looks reasonable")

    if dim_pct < 10:
        print("    ⚠️  LACKS SHADOWS: <10% dim/void cells - may look flat")
    else:
        print("    ✓ Has shadow/dim cells for depth")

    if edge_cells / ink_cells < 0.3:
        print("    ⚠️  LOW EDGE RATIO: may lack anti-aliasing on contours")
    else:
        print("    ✓ Edge ratio suggests anti-aliasing present")

    print()

    # Print the piece for visual reference
    print("  Piece:")
    for line in padded:
        print(f"    {line}")
    print()


def analyze_pack(text: str) -> None:
    """Parse a == name header file and analyze each piece."""
    blocks = text.split("== ")
    for block in blocks:
        block = block.strip()
        if not block:
            continue
        lines = block.split("\n")
        name = lines[0].strip()
        art = lines[1:]
        while art and not art[-1].strip():
            art.pop()
        if not art:
            continue
        analyze_piece(name, art)


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 analyze_art.py <art_file>")
        sys.exit(1)

    path = Path(sys.argv[1])
    if not path.exists():
        print(f"File not found: {path}")
        sys.exit(1)

    text = path.read_text(encoding="utf-8")
    analyze_pack(text)


if __name__ == "__main__":
    main()
