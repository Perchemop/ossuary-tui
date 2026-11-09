//! Per-cell exponential moving average smoothing for rendered frames.
//!
//! [`FramePolish`] blends the previous frame towards the current one so rain
//! trails and art transitions settle between cells instead of snapping. Only
//! the two discrete parts of a [`Cell`] are touched — the glyph and the
//! brightness band — and every output cell reuses a glyph from one of the two
//! inputs and a band from the renderer's existing ramp (see [`MAX_BAND`]), so
//! the layer can never invent characters or off-ramp colours (AC-4).
//!
//! The module is pure: no I/O, no interior state, and no panics. It is a
//! stateless pair blend, so the caller keeps the previous frame and hands both
//! frames in on every call.

use ossuary_core::{Cell, Frame};

/// Highest brightness band the renderer draws.
///
/// Bands are `0` empty, `1` dim tail, `2` mid trail and `3` bright head.
/// [`ossuary_core::render_svg`] paints `1..=3` from a fixed ramp and falls back
/// to the brightest colour for anything else, which would be an off-ramp
/// value. Blended bands are always clamped into `0..=MAX_BAND`.
pub const MAX_BAND: u8 = 3;

/// Per-cell exponential moving average smoother for [`Frame`]s.
///
/// Construct with a gain in `0.0..=1.0` and call [`FramePolish::apply`] with
/// the previous and current frames:
///
/// ```
/// use ossuary_core::{Cell, Frame};
/// use ossuary_sim_polish::FramePolish;
///
/// let mut prev = Frame::new(1, 1);
/// prev.set(0, 0, Cell::EMPTY);
/// let mut curr = Frame::new(1, 1);
/// curr.set(0, 0, Cell { ch: '#', intensity: 3 });
///
/// let polish = FramePolish::new(0.5);
/// let blended = polish.apply(&prev, &curr);
/// assert_eq!(blended.cell(0, 0), Cell { ch: '#', intensity: 2 });
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FramePolish {
    /// Blend gain, always inside `0.0..=1.0`.
    strength: f32,
}

impl FramePolish {
    /// Create a smoother whose gain is `strength`.
    ///
    /// The gain is clamped to `0.0..=1.0`. A `NaN` gain is treated as `0.0`
    /// (no gain, hold the previous frame) rather than being stored, so a bad
    /// value can never poison later frames; there is no panic path.
    pub fn new(strength: f32) -> Self {
        let strength = if strength.is_nan() {
            0.0
        } else {
            strength.clamp(0.0, 1.0)
        };
        Self { strength }
    }

    /// The clamped gain, in `0.0..=1.0`.
    pub fn strength(&self) -> f32 {
        self.strength
    }

    /// Blend `curr` into `prev` and return the smoothed frame.
    ///
    /// Every cell performs the exponential moving average
    /// `result = prev + strength * (curr - prev)` on its brightness band. The
    /// blended band is rounded to the nearest integer inside `0..=MAX_BAND` and
    /// paired with the glyph of whichever input cell the blend landed nearest
    /// to, so the output stays inside the input domain:
    ///
    /// * at `strength == 0.0` the previous frame comes back unchanged — an EMA
    ///   with zero gain holds `prev` — and `apply(frame, frame)` is
    ///   byte-identical to `frame`;
    /// * at `strength == 1.0` the current frame comes back unchanged, assuming
    ///   its cells are already in domain;
    /// * in between, the band moves smoothly with the gain while the glyph
    ///   hands over at the halfway point; no character is ever invented.
    ///
    /// The result takes `curr`'s dimensions. `prev` is read with an empty cell
    /// outside its bounds, so a resized frame fades in from the background
    /// instead of failing.
    #[must_use]
    pub fn apply(&self, prev: &Frame, curr: &Frame) -> Frame {
        let width = curr.width();
        let height = curr.height();
        let mut out = Frame::new(width, height);
        for y in 0..height {
            for x in 0..width {
                out.set(x, y, self.blend_cell(prev.cell(x, y), curr.cell(x, y)));
            }
        }
        out
    }

    /// Exponential moving average of one cell, quantised back to the domain.
    ///
    /// Identical inputs come back out unchanged through the same path: the
    /// blend equals the shared band, which rounds to itself, and the glyph
    /// tie-break keeps a source cell that is equal either way.
    fn blend_cell(&self, prev: Cell, curr: Cell) -> Cell {
        let from = f32::from(prev.intensity);
        let to = f32::from(curr.intensity);
        let blended = from + self.strength * (to - from);
        // Round to the nearest existing band. The blend of two in-domain bands
        // can never leave `0..=MAX_BAND`; the clamp also keeps a caller-supplied
        // off-ramp band honest instead of passing it through.
        let band = blended.clamp(0.0, f32::from(MAX_BAND)).round() as u8;
        if band == 0 {
            return Cell::EMPTY;
        }
        // Keep the glyph of the source cell the blend landed nearest to; a
        // glyph-less source borrows the other glyph so a fading-in cell shows
        // its faint band with real ink.
        let (keeper, other) = if self.nearest_is_prev(from, to, blended) {
            (prev, curr)
        } else {
            (curr, prev)
        };
        let ch = if keeper.ch == ' ' {
            other.ch
        } else {
            keeper.ch
        };
        if ch == ' ' {
            // A space carries no ink, so a glyph-less cell stays canonical.
            return Cell::EMPTY;
        }
        Cell { ch, intensity: band }
    }

    /// Whether the blended band landed closer to `prev` than to `curr`.
    ///
    /// With two different bands the cross-over is exactly at `strength == 0.5`
    /// (the blend is linear in the gain). When both bands are equal the EMA has
    /// no geometric preference, so the gain decides: `prev` below the halfway
    /// mark, `curr` from it up. That keeps `strength == 0.0` reproducing `prev`
    /// and `strength == 1.0` reproducing `curr`.
    fn nearest_is_prev(&self, from: f32, to: f32, blended: f32) -> bool {
        let to_prev = (blended - from).abs();
        let to_curr = (blended - to).abs();
        if (to_prev - to_curr).abs() <= f32::EPSILON {
            self.strength < 0.5
        } else {
            to_prev < to_curr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FramePolish, MAX_BAND};
    use ossuary_core::{Cell, Frame};

    /// Glyphs the fixtures draw from — the whole alphabet the polish layer is
    /// allowed to emit.
    const GLYPHS: [char; 5] = ['0', '#', '|', '.', 'x'];
    /// The renderer's brightness ramp.
    const BANDS: [u8; 4] = [0, 1, 2, 3];

    /// A deterministic frame with a mix of empty cells, glyphs and bands.
    ///
    /// A tiny LCG keeps the fixture varied without pulling in `ossuary-core`'s
    /// RNG or any dependency; `seed` selects a different cell pattern.
    fn sample(width: usize, height: usize, seed: usize) -> Frame {
        let mut frame = Frame::new(width, height);
        let mut state = seed.wrapping_mul(2_654_435_761);
        for y in 0..height {
            for x in 0..width {
                state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                let band = BANDS[(state >> 4) % BANDS.len()];
                let ch = GLYPHS[(state >> 7) % GLYPHS.len()];
                let cell = if band == 0 {
                    Cell::EMPTY
                } else {
                    Cell { ch, intensity: band }
                };
                frame.set(x, y, cell);
            }
        }
        frame
    }

    /// Two small frames that differ in every cell, for hand-checked cases.
    fn opposing_pair() -> (Frame, Frame) {
        let mut prev = Frame::new(2, 1);
        prev.set(0, 0, Cell::EMPTY);
        prev.set(1, 0, Cell { ch: '|', intensity: 3 });
        let mut curr = Frame::new(2, 1);
        curr.set(0, 0, Cell { ch: '#', intensity: MAX_BAND });
        curr.set(1, 0, Cell::EMPTY);
        (prev, curr)
    }

    /// Assert every cell is drawn from the fixture glyphs and the band ramp.
    fn assert_in_domain(frame: &Frame) {
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                let cell = frame.cell(x, y);
                assert!(BANDS.contains(&cell.intensity), "band {} at ({x}, {y})", cell.intensity);
                if cell.intensity == 0 {
                    // Nothing may be drawn in an empty cell.
                    assert_eq!(cell, Cell::EMPTY, "empty cell at ({x}, {y})");
                } else {
                    assert!(GLYPHS.contains(&cell.ch), "glyph {:?} at ({x}, {y})", cell.ch);
                }
            }
        }
    }

    #[test]
    fn frame_polish_identity_at_zero_strength() {
        let frame = sample(12, 7, 3);
        let polish = FramePolish::new(0.0);
        let polished = polish.apply(&frame, &frame);
        assert_eq!(polished.width(), frame.width());
        assert_eq!(polished.height(), frame.height());
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                assert_eq!(polished.cell(x, y), frame.cell(x, y));
            }
        }
        // Whole-frame equality also covers the backing cell storage.
        assert_eq!(polished, frame);
        assert_eq!(polish.strength(), 0.0);
    }

    #[test]
    fn frame_polish_identical_frames_pass_through_any_strength() {
        let frame = sample(6, 4, 21);
        for strength in [0.0, 0.2, 0.5, 0.8, 1.0] {
            let polished = FramePolish::new(strength).apply(&frame, &frame);
            assert_eq!(polished, frame, "changed at strength {strength}");
        }
    }

    #[test]
    fn frame_polish_zero_strength_holds_previous_frame() {
        let prev = sample(9, 5, 11);
        let mut curr = sample(9, 5, 11);
        // Force known differences so the test cannot pass on equal fixtures.
        curr.set(3, 2, Cell { ch: '#', intensity: MAX_BAND });
        curr.set(8, 4, Cell::EMPTY);
        assert_ne!(prev, curr);
        let polished = FramePolish::new(0.0).apply(&prev, &curr);
        assert_eq!(polished, prev);
    }

    #[test]
    fn frame_polish_stays_in_glyph_and_band_domain() {
        let prev = sample(10, 6, 1);
        let curr = sample(10, 6, 97);
        assert_ne!(prev, curr);
        for strength in [0.0, 0.1, 0.25, 0.4, 0.5, 0.66, 0.75, 0.9, 1.0] {
            let polished = FramePolish::new(strength).apply(&prev, &curr);
            assert_eq!(polished.width(), prev.width());
            assert_eq!(polished.height(), prev.height());
            assert_in_domain(&polished);
        }
    }

    #[test]
    fn frame_polish_full_strength_reproduces_current() {
        let prev = sample(8, 4, 5);
        let curr = sample(8, 4, 6);
        let polished = FramePolish::new(1.0).apply(&prev, &curr);
        assert_eq!(polished, curr);
    }

    #[test]
    fn frame_polish_band_moves_monotonically_with_strength() {
        let prev = Frame::new(1, 1);
        let mut curr = Frame::new(1, 1);
        curr.set(0, 0, Cell { ch: '#', intensity: MAX_BAND });
        let mut last = 0;
        for step in 0..=20u8 {
            let strength = f32::from(step) / 20.0;
            let cell = FramePolish::new(strength).apply(&prev, &curr).cell(0, 0);
            assert!(cell.intensity >= last, "band fell to {}", cell.intensity);
            last = cell.intensity;
        }
        let none = FramePolish::new(0.0).apply(&prev, &curr);
        let full = FramePolish::new(1.0).apply(&prev, &curr);
        assert_eq!(none.cell(0, 0), Cell::EMPTY);
        assert_eq!(full.cell(0, 0), Cell { ch: '#', intensity: MAX_BAND });
    }

    #[test]
    fn frame_polish_glyph_hands_over_at_halfway() {
        let mut prev = Frame::new(1, 1);
        prev.set(0, 0, Cell { ch: '.', intensity: 2 });
        let mut curr = Frame::new(1, 1);
        curr.set(0, 0, Cell { ch: '#', intensity: 2 });
        // Equal bands: the gain decides, so the glyph swaps at 0.5.
        let below = FramePolish::new(0.49).apply(&prev, &curr);
        let at = FramePolish::new(0.5).apply(&prev, &curr);
        assert_eq!(below.cell(0, 0), Cell { ch: '.', intensity: 2 });
        assert_eq!(at.cell(0, 0), Cell { ch: '#', intensity: 2 });
    }

    #[test]
    fn frame_polish_takes_current_dimensions() {
        let prev = sample(2, 2, 1);
        let mut curr = Frame::new(4, 3);
        curr.set(0, 0, Cell { ch: '#', intensity: MAX_BAND });
        let polished = FramePolish::new(1.0).apply(&prev, &curr);
        assert_eq!(polished.width(), 4);
        assert_eq!(polished.height(), 3);
        assert_eq!(polished.cell(0, 0), Cell { ch: '#', intensity: MAX_BAND });
        // Cells only `prev` knows about blend towards the empty background.
        let shrunk = FramePolish::new(1.0).apply(&prev, &Frame::new(2, 2));
        assert_eq!(shrunk, Frame::new(2, 2));
    }

    #[test]
    fn frame_polish_clamps_strength() {
        assert_eq!(FramePolish::new(-3.0).strength(), 0.0);
        assert_eq!(FramePolish::new(0.0).strength(), 0.0);
        assert_eq!(FramePolish::new(0.35).strength(), 0.35);
        assert_eq!(FramePolish::new(1.0).strength(), 1.0);
        assert_eq!(FramePolish::new(4.2).strength(), 1.0);
        assert_eq!(FramePolish::new(f32::INFINITY).strength(), 1.0);
        assert_eq!(FramePolish::new(f32::NEG_INFINITY).strength(), 0.0);
        assert_eq!(FramePolish::new(f32::NAN).strength(), 0.0);
    }

    #[test]
    fn frame_polish_clamps_off_ramp_bands() {
        let mut frame = Frame::new(2, 1);
        frame.set(0, 0, Cell { ch: '#', intensity: 9 });
        frame.set(1, 0, Cell { ch: '.', intensity: 200 });
        let polished = FramePolish::new(0.5).apply(&frame, &frame);
        assert_in_domain(&polished);
        assert_eq!(polished.cell(0, 0), Cell { ch: '#', intensity: MAX_BAND });
        assert_eq!(polished.cell(1, 0), Cell { ch: '.', intensity: 3 });
    }

    #[test]
    fn frame_polish_opposing_pair_blends_towards_current() {
        let (prev, curr) = opposing_pair();
        let quarter = FramePolish::new(0.25).apply(&prev, &curr);
        let three_quarters = FramePolish::new(0.75).apply(&prev, &curr);
        // Rising cell: empty -> band 3 shows a faint glyph before halfway.
        assert_eq!(quarter.cell(0, 0), Cell { ch: '#', intensity: 1 });
        assert_eq!(three_quarters.cell(0, 0), Cell { ch: '#', intensity: 2 });
        // Falling cell: band 3 -> empty keeps the dimming glyph until it dies.
        assert_eq!(quarter.cell(1, 0), Cell { ch: '|', intensity: 2 });
        assert_eq!(three_quarters.cell(1, 0), Cell { ch: '|', intensity: 1 });
        assert_in_domain(&quarter);
        assert_in_domain(&three_quarters);
    }
}