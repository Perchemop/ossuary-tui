//! Hand-typed ASCII art pieces and the library that holds them.
//!
//! Every piece that ships with Ossuary was typed by hand for this project in
//! the scene's *solid/grayscale* tradition: a bold fill character with a long
//! tone ramp carved inside it, eye sockets and other voids cut out as dark
//! regions, and the silhouette formed by anti-aliasing the fill's edge. No
//! piece is copied from another artist (guide §2.2).
//!
//! The built-in pack lives in `assets/art/pieces.txt` and is embedded at
//! compile time, so art can be edited as data and previewed without building.

use std::path::Path;

use crate::error::CoreError;

/// The built-in pack, embedded at compile time.
const BUILT_IN_TEXT: &str = include_str!("../../../assets/art/pieces.txt");

/// Marker that starts a new piece in the embedded pack file.
const HEADER: &str = "== ";

/// Name prefix marking an embedded piece as a text phrase rather than a
/// picture.
const PHRASE_PREFIX: &str = "phrase";

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

    /// The `index`-th inked cell in reading order, for the typewriter reveal.
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

    /// The picture pieces that ship with Ossuary.
    pub fn built_in() -> Self {
        let (pictures, _) = parse_pack(BUILT_IN_TEXT);
        Self::new(pictures)
    }

    /// The text phrases that ship with Ossuary.
    pub fn built_in_phrases() -> Self {
        let (_, phrases) = parse_pack(BUILT_IN_TEXT);
        Self::new(phrases)
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
}

/// Everything the rain can summon: pictures plus text phrases.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArtLibrary {
    /// Hand-typed pictures.
    pub pictures: ArtPack,
    /// Short typed phrases (shown in the same typewriter reveal).
    pub phrases: ArtPack,
}

impl ArtLibrary {
    /// The original library that ships with Ossuary.
    pub fn built_in() -> Self {
        let (pictures, phrases) = parse_pack(BUILT_IN_TEXT);
        Self {
            pictures: ArtPack::new(pictures),
            phrases: ArtPack::new(phrases),
        }
    }

    /// An empty library.
    pub fn empty() -> Self {
        Self::default()
    }

    /// The built-in library with a user pack appended from `dir`.
    pub fn built_in_with_user_art(dir: &Path) -> Self {
        let mut library = Self::built_in();
        if let Ok(user) = ArtPack::load_dir(dir) {
            library.pictures.extend(user);
        }
        library
    }

    /// Look up a piece by name across either pictures or phrases.
    pub fn named(&self, name: &str, phrase: bool) -> Option<usize> {
        if phrase {
            self.phrases.index_of(name)
        } else {
            self.pictures.index_of(name)
        }
    }

    /// Whether nothing at all can be summoned.
    pub fn is_empty(&self) -> bool {
        self.pictures.is_empty() && self.phrases.is_empty()
    }
}

/// Brightness of an art glyph, for a bright-ink-on-dark-terminal render.
///
/// Art carries its own shading in the *choice* of character, exactly as
/// scene-style solid/grayscale work does: dense fill characters read as lit
/// bone, sparse punctuation reads as shadow, and space is void. The renderer
/// maps each glyph to one of the terminal's brightness bands, so a piece keeps
/// its tonal range instead of flattening into a single colour.
pub fn tone(ch: char) -> u8 {
    match ch {
        // Dense fill: the brightest things on screen.
        '$' | '#' | '8' | '&' | '@' | '%' | 'W' | 'M' | 'B' | '0' | 'Q' | 'D' => 3,
        // Edge and shadow work: deliberately dim.
        '.' | ',' | '\'' | '"' | '`' | '^' | '~' | '-' | '_' | ':' | ';' => 1,
        // Void.
        ' ' => 0,
        // Everything else sits in the middle band.
        _ => 2,
    }
}

/// Split the embedded pack text into picture pieces and phrase pieces.
fn parse_pack(text: &str) -> (Vec<ArtPiece>, Vec<ArtPiece>) {
    let mut pictures = Vec::new();
    let mut phrases = Vec::new();
    let mut name: Option<String> = None;
    let mut lines: Vec<String> = Vec::new();

    for raw in text.lines() {
        if let Some(header) = raw.strip_prefix(HEADER) {
            if let Some(current) = name.take() {
                push_piece(&mut pictures, &mut phrases, current, &lines);
            }
            name = Some(header.trim().to_string());
            lines.clear();
        } else if name.is_some() {
            lines.push(raw.trim_end_matches('\r').to_string());
        }
    }
    if let Some(current) = name.take() {
        push_piece(&mut pictures, &mut phrases, current, &lines);
    }
    (pictures, phrases)
}

/// Trim trailing blank lines and file the piece under pictures or phrases.
fn push_piece(
    pictures: &mut Vec<ArtPiece>,
    phrases: &mut Vec<ArtPiece>,
    name: String,
    lines: &[String],
) {
    let mut end = lines.len();
    while end > 0 && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    let refs: Vec<&str> = lines[..end].iter().map(String::as_str).collect();
    let piece = ArtPiece::from_lines(name, &refs);
    if piece.is_empty() {
        return;
    }
    if piece.name().starts_with(PHRASE_PREFIX) {
        phrases.push(piece);
    } else {
        pictures.push(piece);
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
    fn built_in_pictures_are_dense_and_well_formed() {
        let pack = ArtPack::built_in();
        assert!(pack.len() >= 5);
        for name in ["skull", "crossbones", "cat", "candle", "moon", "ghost"] {
            assert!(pack.index_of(name).is_some(), "missing built-in piece {name}");
        }
        for piece in pack.pieces() {
            assert!(!piece.is_empty(), "piece {} must have ink", piece.name());
            // Density is the point: a piece this size should carry real ink.
            assert!(
                piece.ink_count() >= 30,
                "piece {} is too sparse ({} ink)",
                piece.name(),
                piece.ink_count()
            );
        }
    }

    #[test]
    fn built_in_phrases_exist() {
        let phrases = ArtPack::built_in_phrases();
        assert!(!phrases.is_empty());
        for piece in phrases.pieces() {
            assert!(piece.name().starts_with(PHRASE_PREFIX));
        }
    }

    #[test]
    fn library_splits_pictures_from_phrases() {
        let library = ArtLibrary::built_in();
        assert!(!library.pictures.is_empty());
        assert!(!library.phrases.is_empty());
        // A phrase never appears in the picture pack.
        for phrase in library.phrases.pieces() {
            assert!(library.pictures.index_of(phrase.name()).is_none());
        }
    }

    #[test]
    fn tone_puts_dense_fill_bright_and_edges_dim() {
        assert_eq!(tone('$'), 3);
        assert_eq!(tone('8'), 3);
        assert_eq!(tone('S'), 2);
        assert_eq!(tone('I'), 2);
        assert_eq!(tone(':'), 1);
        assert_eq!(tone('.'), 1);
        assert_eq!(tone(' '), 0);
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

    #[test]
    fn parse_pack_splits_on_headers_and_drops_blank_tail() {
        let text = "== alpha\nxx\n\n\n== beta\nyy\n";
        let (pictures, phrases) = parse_pack(text);
        assert_eq!(pictures.len(), 2);
        assert!(phrases.is_empty());
        assert_eq!(pictures[0].height(), 1, "trailing blank lines are dropped");
        assert_eq!(pictures[0].name(), "alpha");
    }
}
