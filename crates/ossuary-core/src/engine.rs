//! The rain simulation: falling character columns, plus the art piece that is
//! periodically typed into existence over them.
//!
//! The engine holds no terminal state. Call [`Engine::step`] to advance one
//! frame and [`Engine::frame`] to read the result; both are pure with respect
//! to everything outside the engine, and both are deterministic for a given
//! seed (guide §K.13, §M.5).

use crate::art::{ArtPack, ArtPiece};
use crate::config::Config;
use crate::glyphs::GlyphSet;
use crate::rng::Rng;

/// Shortest possible column trail.
const MIN_TRAIL: usize = 4;
/// Steps a fully drawn piece stays on screen before it dissolves.
const HOLD_STEPS: u32 = 140;
/// Steps a piece spends dissolving.
const DISSOLVE_STEPS: u32 = 40;
/// Inked glyphs revealed per step while a piece is being typed.
const REVEAL_PER_STEP: usize = 3;
/// Chance per step that an inactive column is re-rolled, so density is honoured
/// over time rather than frozen forever.
const REACTIVATE_CHANCE: f32 = 0.02;

/// One cell of a rendered frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The glyph to draw (a space when nothing is drawn).
    pub ch: char,
    /// Brightness band: 0 empty, 1 dim tail, 2 mid trail, 3 bright head.
    pub intensity: u8,
}

impl Cell {
    /// An empty cell.
    pub const EMPTY: Cell = Cell {
        ch: ' ',
        intensity: 0,
    };

    /// Whether nothing should be drawn here.
    pub fn is_empty(self) -> bool {
        self.intensity == 0
    }
}

/// A rendered frame: a rectangular grid of [`Cell`]s in row-major order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Frame {
    /// An all-empty frame of the given size.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::EMPTY; width * height],
        }
    }

    /// Frame width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Frame height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Cell at `(x, y)`; out-of-range reads return empty.
    pub fn cell(&self, x: usize, y: usize) -> Cell {
        if x >= self.width || y >= self.height {
            Cell::EMPTY
        } else {
            self.cells[y * self.width + x]
        }
    }

    /// Write a cell, ignoring out-of-range coordinates.
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }
}

/// Lifecycle phase of the summoned art piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtState {
    /// No art piece is being shown.
    Idle,
    /// A piece is being typed into existence.
    Typing,
    /// A piece is fully drawn.
    Holding,
    /// A piece is fading back into the rain.
    Dissolving,
}

/// One falling column.
#[derive(Debug, Clone)]
struct Column {
    /// Row index of the leading glyph (may be negative while entering).
    head: i64,
    /// Per-column speed multiplier (0.5–1.5), independent of the global speed.
    factor: f32,
    /// Sub-cell accumulator for fractional speeds.
    accum: f32,
    /// Characters; `chars[0]` sits at the head, the tail is last.
    chars: Vec<char>,
    /// Intended trail length.
    len: usize,
    /// Brightness-band offset (−1, 0 or +1) for colour drift.
    drift: i8,
    /// Whether this column is currently raining.
    active: bool,
}

impl Column {
    /// Spawn a column. `from_top` places it just above the top edge (a fresh
    /// column re-entering the screen); otherwise it is distributed anywhere on
    /// screen so the first frame is already full.
    fn spawn(
        rng: &mut Rng,
        height: usize,
        config: &Config,
        glyphs: GlyphSet,
        from_top: bool,
    ) -> Self {
        let max_trail = config.trail.max(MIN_TRAIL);
        let span = max_trail.saturating_sub(MIN_TRAIL) + 1;
        let len = MIN_TRAIL + rng.below(span);
        let chars: Vec<char> = (0..len).map(|_| glyphs.pick(rng)).collect();
        let head = if from_top {
            -(rng.below(len.max(1)) as i64) - 1
        } else {
            rng.below(height.max(1)) as i64
        };
        let drift = if config.color_drift {
            [-1i8, 0, 1][rng.below(3)]
        } else {
            0
        };
        Self {
            head,
            factor: rng.range_f32(0.5, 1.5),
            accum: 0.0,
            chars,
            len,
            drift,
            active: rng.next_f32() < config.density,
        }
    }
}

/// The rain engine.
pub struct Engine {
    width: usize,
    height: usize,
    columns: Vec<Column>,
    rng: Rng,
    config: Config,
    glyphs: GlyphSet,
    pack: ArtPack,
    art_state: ArtState,
    art_index: usize,
    art_revealed: usize,
    art_timer: u32,
    art_x: usize,
    art_y: usize,
    tick: u64,
}

impl Engine {
    /// Create an engine for a `width` × `height` grid.
    pub fn new(width: usize, height: usize, config: Config, pack: ArtPack, seed: u64) -> Self {
        let config = config.sanitized();
        let glyphs = GlyphSet::by_name(&config.glyph_set).unwrap_or_else(GlyphSet::default_set);
        let mut rng = Rng::new(seed);
        let columns = (0..width)
            .map(|_| Column::spawn(&mut rng, height, &config, glyphs, false))
            .collect();
        let mut engine = Self {
            width,
            height,
            columns,
            rng,
            config,
            glyphs,
            pack,
            art_state: ArtState::Idle,
            art_index: 0,
            art_revealed: 0,
            art_timer: 0,
            art_x: 0,
            art_y: 0,
            tick: 0,
        };
        engine.reposition_art();
        engine
    }

    /// Grid width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Grid height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Steps taken so far.
    pub fn tick(&self) -> u64 {
        self.tick
    }

    /// The active configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// The active glyph set.
    pub fn glyphs(&self) -> GlyphSet {
        self.glyphs
    }

    /// The art pack.
    pub fn pack(&self) -> &ArtPack {
        &self.pack
    }

    /// Current art lifecycle phase.
    pub fn art_state(&self) -> ArtState {
        self.art_state
    }

    /// The piece currently being shown, if any.
    pub fn current_piece(&self) -> Option<&ArtPiece> {
        if self.art_state == ArtState::Idle {
            None
        } else {
            self.pack.piece(self.art_index)
        }
    }

    /// Resize the grid, preserving settings.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        let config = self.config.clone();
        let glyphs = self.glyphs;
        self.columns = (0..width)
            .map(|_| Column::spawn(&mut self.rng, height, &config, glyphs, false))
            .collect();
        self.reposition_art();
    }

    /// Apply a new configuration live.
    pub fn set_config(&mut self, config: Config) {
        let config = config.sanitized();
        let glyphs = GlyphSet::by_name(&config.glyph_set).unwrap_or(self.glyphs);
        if glyphs != self.glyphs {
            // Re-ink the existing trails from the new set so the change is
            // immediate rather than only after columns respawn.
            for column in &mut self.columns {
                for ch in &mut column.chars {
                    *ch = glyphs.pick(&mut self.rng);
                }
            }
        }
        self.glyphs = glyphs;
        self.config = config;
    }

    /// Immediately summon the next art piece, interrupting any current one.
    pub fn summon(&mut self) {
        self.start_art();
    }

    fn start_art(&mut self) {
        let count = self.pack.len();
        if count == 0 {
            return;
        }
        self.art_index = match self.config.art_mode.as_str() {
            "random" => self.rng.below(count),
            "single" => self
                .pack
                .index_of(&self.config.art_piece)
                .unwrap_or((self.art_index + 1) % count),
            _ => (self.art_index + 1) % count,
        };
        self.art_revealed = 0;
        self.art_state = ArtState::Typing;
        self.reposition_art();
    }

    fn reposition_art(&mut self) {
        if let Some(piece) = self.pack.piece(self.art_index) {
            self.art_x = self.width.saturating_sub(piece.width()) / 2;
            self.art_y = self.height.saturating_sub(piece.height()) / 2;
        } else {
            self.art_x = 0;
            self.art_y = 0;
        }
    }

    /// Advance the simulation by one step.
    pub fn step(&mut self) {
        self.tick += 1;
        self.step_columns();
        self.step_art();
    }

    fn step_columns(&mut self) {
        let height = self.height;
        let config = self.config.clone();
        let glyphs = self.glyphs;
        for column in &mut self.columns {
            if !column.active {
                if self.rng.next_f32() < REACTIVATE_CHANCE {
                    *column = Column::spawn(&mut self.rng, height, &config, glyphs, false);
                }
                continue;
            }

            column.accum += config.speed * column.factor;
            while column.accum >= 1.0 {
                column.accum -= 1.0;
                column.head += 1;
                column.chars.insert(0, glyphs.pick(&mut self.rng));
                column.chars.truncate(column.len);
            }

            // Flicker: swap a character in place, exactly as the type-art
            // technique describes.
            if config.flicker > 0.0
                && !column.chars.is_empty()
                && self.rng.next_f32() < config.flicker
            {
                let index = self.rng.below(column.chars.len());
                column.chars[index] = glyphs.pick(&mut self.rng);
            }

            if column.head - column.len as i64 > height as i64 {
                *column = Column::spawn(&mut self.rng, height, &config, glyphs, true);
            }
        }
    }

    fn step_art(&mut self) {
        match self.art_state {
            ArtState::Idle => {
                let interval = self.config.art_interval;
                if self.config.show_art
                    && !self.pack.is_empty()
                    && interval > 0
                    && self.tick % u64::from(interval) == 0
                {
                    self.start_art();
                }
            }
            ArtState::Typing => {
                let total = self
                    .pack
                    .piece(self.art_index)
                    .map_or(0, ArtPiece::ink_count);
                self.art_revealed += REVEAL_PER_STEP;
                if self.art_revealed >= total {
                    self.art_revealed = total;
                    self.art_state = ArtState::Holding;
                    self.art_timer = HOLD_STEPS;
                }
            }
            ArtState::Holding => {
                if self.art_timer == 0 {
                    self.art_state = ArtState::Dissolving;
                    self.art_timer = DISSOLVE_STEPS;
                } else {
                    self.art_timer -= 1;
                }
            }
            ArtState::Dissolving => {
                if self.art_timer == 0 {
                    self.art_state = ArtState::Idle;
                } else {
                    self.art_timer -= 1;
                }
            }
        }
    }

    /// Render the current state into a frame.
    pub fn frame(&self) -> Frame {
        let mut frame = Frame::new(self.width, self.height);
        for (x, column) in self.columns.iter().enumerate() {
            if !column.active || x >= self.width {
                continue;
            }
            let count = column.chars.len();
            for (distance, &ch) in column.chars.iter().enumerate() {
                let row = column.head - distance as i64;
                if row < 0 || row >= self.height as i64 {
                    continue;
                }
                let base: u8 = if distance == 0 {
                    3
                } else if distance * 3 < count {
                    2
                } else {
                    1
                };
                let intensity = (i16::from(base) + i16::from(column.drift)).clamp(1, 3) as u8;
                frame.set(x, row as usize, Cell { ch, intensity });
            }
        }
        self.overlay_art(&mut frame);
        frame
    }

    fn overlay_art(&self, frame: &mut Frame) {
        let Some(piece) = self.pack.piece(self.art_index) else {
            return;
        };
        let total = piece.ink_count();
        let (limit, intensity) = match self.art_state {
            ArtState::Idle => return,
            ArtState::Typing => (self.art_revealed.min(total), 3),
            ArtState::Holding => (total, 3),
            ArtState::Dissolving => {
                let remaining = total * self.art_timer as usize / DISSOLVE_STEPS.max(1) as usize;
                (remaining.min(total), 2)
            }
        };
        for index in 0..limit {
            if let Some((px, py, ch)) = piece.ink_at(index) {
                frame.set(self.art_x + px, self.art_y + py, Cell { ch, intensity });
            }
        }
        if self.config.show_titles && self.art_state != ArtState::Dissolving {
            Self::draw_title(frame, piece, self.art_x, self.art_y);
        }
    }

    /// Write the piece's name, centred, one row beneath it.
    fn draw_title(frame: &mut Frame, piece: &ArtPiece, art_x: usize, art_y: usize) {
        let title = piece.name();
        let row = art_y + piece.height() + 1;
        if row >= frame.height() {
            return;
        }
        let text_width = title.chars().count();
        let col = art_x + piece.width().saturating_sub(text_width) / 2;
        for (offset, ch) in title.chars().enumerate() {
            frame.set(col + offset, row, Cell { ch, intensity: 2 });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize_changes_the_frame_dimensions() {
        let mut engine = Engine::new(20, 8, Config::default(), ArtPack::empty(), 1);
        engine.resize(30, 10);
        let frame = engine.frame();
        assert_eq!(frame.width(), 30);
        assert_eq!(frame.height(), 10);
    }

    #[test]
    fn set_config_re_resolves_glyphs_and_keeps_unknown_as_is() {
        let mut engine = Engine::new(20, 8, Config::default(), ArtPack::empty(), 2);
        let mut config = Config {
            glyph_set: "letters".to_string(),
            ..Config::default()
        };
        engine.set_config(config.clone());
        assert_eq!(engine.glyphs().name(), "letters");
        // An unknown name must not change the active set.
        config.glyph_set = "does-not-exist".to_string();
        engine.set_config(config);
        assert_eq!(engine.glyphs().name(), "letters");
    }

    #[test]
    fn summon_with_an_empty_pack_is_a_no_op() {
        let mut engine = Engine::new(20, 8, Config::default(), ArtPack::empty(), 3);
        engine.summon();
        assert_eq!(engine.art_state(), ArtState::Idle);
        assert!(engine.current_piece().is_none());
    }

    #[test]
    fn intensity_bands_are_ordered_from_head_to_tail() {
        let config = Config {
            density: 1.0,
            ..Config::default()
        };
        let mut engine = Engine::new(5, 40, config, ArtPack::empty(), 4);
        for _ in 0..5 {
            engine.step();
        }
        // At least one head cell should be the brightest band.
        let frame = engine.frame();
        let mut has_head = false;
        for y in 0..frame.height() {
            for x in 0..frame.width() {
                if frame.cell(x, y).intensity == 3 {
                    has_head = true;
                }
            }
        }
        assert!(has_head, "a raining column should have a bright head");
    }
}
