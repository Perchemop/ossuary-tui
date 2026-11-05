//! Colour mapping for the rain's brightness bands.

use crossterm::style::Color;

/// Map a brightness band (`0` empty … `3` bright head) to a terminal colour.
///
/// Monochrome is the house style; the green palette is the classic screen look.
pub fn color_for(intensity: u8, monochrome: bool) -> Color {
    match (monochrome, intensity) {
        (_, 0) => Color::Reset,
        (true, 1) => Color::DarkGrey,
        (true, 2) => Color::Grey,
        (true, _) => Color::White,
        (false, 1) => Color::DarkGreen,
        (false, 2) => Color::Green,
        (false, _) => Color::LightGreen,
    }
}
