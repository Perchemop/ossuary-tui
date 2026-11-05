//! Hand-typed ASCII art pieces and the packs that hold them.
//!
//! Every piece that ships with Ossuary was typed by hand for this project.
//! None is copied from another artist's work (guide §2.2, clean-room rule): the
//! shipped pack is original, and a user may add their own pieces as plain
//! `.txt` files in the art directory.

use std::path::Path;

use crate::error::CoreError;

/// One hand-typed ASCII art piece.
///
/// Lines are normalised to an equal width on construction, so every accessor
/// can index a rectangular grid without bounds gymnastics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtPiece {
    name: String,
    lines: Vec<String>,
    width: usize,
}

impl ArtPiece {
    /// Build a piece from raw lines, right-padding every line to the widest.
    pub fn from_lines(name: impl Into<String>, lines: &[&str]) -> Self {
        let width = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
        let lines = lines
            .iter()
            .map(|line| {
                let mut owned = (*line).to_string();
                let padding = width.saturating_sub(owned.chars().count());
                for _ in 0..padding {
                    owned.push(' ');
                }
                owned
            })
            .collect();
        Self {
            name: name.into(),
            lines,
            width,
        }
    }

    /// Build a piece from a text block (used for user-supplied `.txt` files).
    pub fn from_text(name: impl Into<String>, text: &str) -> Self {
        let lines: Vec<&str> = text.lines().collect();
        Self::from_lines(name, &lines)
    }

    /// The piece's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Width in columns.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height in rows.
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Whether the piece has nothing drawable.
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.lines.is_empty() || self.ink_count() == 0
    }

    /// The line strings (equal width).
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// The glyph at `(x, y)`, or a space when out of range.
    pub fn glyph(&self, x: usize, y: usize) -> char {
        self.lines
            .get(y)
            .and_then(|line| line.chars().nth(x))
            .unwrap_or(' ')
    }

    /// Number of non-space glyphs — the piece's "ink".
    pub fn ink_count(&self) -> usize {
        self.lines
            .iter()
            .flat_map(|line| line.chars())
            .filter(|ch| *ch != ' ')
            .count()
    }

    /// The `index`-th inked cell in reading order, for a typewriter reveal.
    pub fn ink_at(&self, index: usize) -> Option<(usize, usize, char)> {
        let mut seen = 0;
        for (y, line) in self.lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    if seen == index {
                        return Some((x, y, ch));
                    }
                    seen += 1;
                }
            }
        }
        None
    }
}

/// A collection of art pieces that the rain can summon.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArtPack {
    pieces: Vec<ArtPiece>,
}

impl ArtPack {
    /// Build a pack from pieces.
    pub fn new(pieces: Vec<ArtPiece>) -> Self {
        Self { pieces }
    }

    /// An empty pack.
    pub fn empty() -> Self {
        Self::default()
    }

    /// The original pack that ships with Ossuary.
    ///
    /// All six pieces were typed by hand for this project in a sparse,
    /// monochrome punctuation style; none is a copy of another artist's work.
    pub fn built_in() -> Self {
        Self::new(vec![
            ArtPiece::from_lines(
                "skull",
                &[
                    "  _____",
                    " /     \\",
                    "| () () |",
                    "|   ^   |",
                    "|  \\_/  |",
                    " \\_____/ ",
                ],
            ),
            ArtPiece::from_lines(
                "bones",
                &[
                    " \\     /",
                    "  \\   /",
                    "   \\ /",
                    "   / \\",
                    "  /   \\",
                    " /     \\",
                ],
            ),
            ArtPiece::from_lines("cat", &[" /\\_/\\", "( o.o )", " > ^ < "]),
            ArtPiece::from_lines("candle", &["  (*)", "  | |", "  | |", "  |_|"]),
            ArtPiece::from_lines("goose", &["  ___", " (o o)", " /|_|\\", "  ^ ^"]),
            ArtPiece::from_lines("carriage", &["  ____", " |:::_|", "=|_|_|=", " o   o"]),
        ])
    }

    /// Number of pieces.
    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    /// Whether the pack holds no pieces.
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }

    /// The pieces.
    pub fn pieces(&self) -> &[ArtPiece] {
        &self.pieces
    }

    /// A piece by position.
    pub fn piece(&self, index: usize) -> Option<&ArtPiece> {
        self.pieces.get(index)
    }

    /// Position of a piece by name.
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.pieces.iter().position(|piece| piece.name == name)
    }

    /// Add a piece and return its index.
    pub fn push(&mut self, piece: ArtPiece) -> usize {
        self.pieces.push(piece);
        self.pieces.len() - 1
    }

    /// Append every non-empty piece from another pack.
    pub fn extend(&mut self, other: ArtPack) {
        for piece in other.pieces {
            if !piece.is_empty() {
                self.pieces.push(piece);
            }
        }
    }

    /// Load a user pack: every `*.txt` file in `dir` becomes a piece whose name
    /// is the file stem. A missing directory yields an empty pack rather than an
    /// error, so a fresh install with no user art still starts.
    pub fn load_dir(dir: &Path) -> Result<Self, CoreError> {
        if !dir.is_dir() {
            return Ok(Self::default());
        }
        let mut entries: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("txt"))
            })
            .collect();
        entries.sort();
        let mut pieces = Vec::new();
        for path in entries {
            let name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| CoreError::InvalidPath(path.clone()))?
                .to_string();
            let text = std::fs::read_to_string(&path)?;
            let piece = ArtPiece::from_text(name, &text);
            if !piece.is_empty() {
                pieces.push(piece);
            }
        }
        Ok(Self::new(pieces))
    }

    /// The built-in pack with a user pack appended from `dir`.
    pub fn built_in_with_user_art(dir: &Path) -> Self {
        let mut pack = Self::built_in();
        if let Ok(user) = Self::load_dir(dir) {
            pack.extend(user);
        }
        pack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_lines_pads_every_line_to_the_widest() {
        let piece = ArtPiece::from_lines("t", &["ab", "abcd", "a"]);
        assert_eq!(piece.width(), 4);
        assert_eq!(piece.height(), 3);
        for line in piece.lines() {
            assert_eq!(line.chars().count(), 4);
        }
        assert_eq!(piece.glyph(3, 2), ' ');
        assert_eq!(piece.glyph(1, 0), 'b');
    }

    #[test]
    fn glyph_out_of_range_is_a_space() {
        let piece = ArtPiece::from_lines("t", &["x"]);
        assert_eq!(piece.glyph(9, 9), ' ');
    }

    #[test]
    fn ink_count_and_ink_at_agree() {
        let piece = ArtPiece::from_lines("t", &["a b", " c "]);
        assert_eq!(piece.ink_count(), 3);
        let mut seen = Vec::new();
        for index in 0..piece.ink_count() {
            seen.push(piece.ink_at(index).expect("ink cell"));
        }
        assert_eq!(seen, vec![(0, 0, 'a'), (2, 0, 'b'), (1, 1, 'c')]);
        assert!(piece.ink_at(piece.ink_count()).is_none());
    }

    #[test]
    fn blank_piece_is_empty() {
        assert!(ArtPiece::from_lines("blank", &["   ", "   "]).is_empty());
    }

    #[test]
    fn built_in_pack_is_populated_and_well_formed() {
        let pack = ArtPack::built_in();
        assert!(pack.len() >= 4);
        assert!(pack.index_of("skull").is_some());
        for piece in pack.pieces() {
            assert!(!piece.is_empty(), "piece {} must have ink", piece.name());
            assert!(piece.ink_count() > 0);
        }
    }

    #[test]
    fn user_pack_loads_txt_files_and_ignores_the_rest() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("arrow.txt"), " -->\n  |\n").expect("write");
        std::fs::write(dir.path().join("readme.md"), "not art").expect("write");
        let pack = ArtPack::load_dir(dir.path()).expect("load");
        assert_eq!(pack.len(), 1);
        assert_eq!(pack.piece(0).expect("piece").name(), "arrow");
    }

    #[test]
    fn missing_user_directory_is_not_an_error() {
        let dir = tempfile::tempdir().expect("tempdir");
        let missing = dir.path().join("does-not-exist");
        assert!(ArtPack::load_dir(&missing).expect("load").is_empty());
    }
}
