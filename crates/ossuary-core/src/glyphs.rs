//! The character sets that may fall as rain.
//!
//! The default set is deliberately thin punctuation rather than the katakana of
//! a classic matrix screen: Ossuary's house style is the sparse, monochrome,
//! hand-typed look of punctuation-based type art.

use crate::rng::Rng;

/// A named set of characters that may fall as rain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphSet {
    name: &'static str,
    chars: &'static str,
}

impl GlyphSet {
    /// The set's stable name (used in [`crate::Config::glyph_set`]).
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The characters, in order.
    pub const fn chars(self) -> &'static str {
        self.chars
    }

    /// Number of characters in the set.
    pub fn len(self) -> usize {
        self.chars.chars().count()
    }

    /// Whether the set is empty.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// The `index`-th character, wrapping around.
    pub fn char_at(self, index: usize) -> char {
        let count = self.len();
        if count == 0 {
            return ' ';
        }
        self.chars.chars().nth(index % count).unwrap_or(' ')
    }

    /// A uniformly random character from the set.
    pub fn pick(self, rng: &mut Rng) -> char {
        self.char_at(rng.below(self.len()))
    }

    /// Look a set up by name.
    pub fn by_name(name: &str) -> Option<Self> {
        BUILT_IN.iter().copied().find(|set| set.name == name)
    }

    /// The set shipped as the default.
    pub fn default_set() -> Self {
        BUILT_IN[0]
    }

    /// All built-in set names, in cycle order.
    pub fn names() -> impl Iterator<Item = &'static str> {
        BUILT_IN.iter().map(|set| set.name)
    }

    /// The next set after this one, in cycle order.
    pub fn cycled(self) -> Self {
        match BUILT_IN.iter().position(|set| set.name == self.name) {
            Some(index) => BUILT_IN[(index + 1) % BUILT_IN.len()],
            None => BUILT_IN[0],
        }
    }
}

/// The built-in glyph sets. The first is the default.
pub const BUILT_IN: [GlyphSet; 5] = [
    // Thin punctuation: the house style, close to how punctuation type art
    // shades with dots, colons, slashes and dashes.
    GlyphSet {
        name: "bone",
        chars: ".,:;'`\"-_=+*/\\|iIlvxX()[]{}<>",
    },
    // A narrow brightness ramp of shading characters.
    GlyphSet {
        name: "shade",
        chars: " .:-=+*#%@",
    },
    // Digits only.
    GlyphSet {
        name: "digits",
        chars: "0123456789",
    },
    // Upper-case letters only.
    GlyphSet {
        name: "letters",
        chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    },
    // A wide slice of printable ASCII.
    GlyphSet {
        name: "ascii",
        chars: "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_set_is_the_first_built_in() {
        assert_eq!(GlyphSet::default_set(), BUILT_IN[0]);
        assert_eq!(GlyphSet::default_set().name(), "bone");
    }

    #[test]
    fn every_built_in_is_non_empty_and_uniquely_named() {
        for (index, set) in BUILT_IN.iter().enumerate() {
            assert!(!set.is_empty(), "set {index} should not be empty");
            assert!(
                GlyphSet::by_name(set.name()).is_some(),
                "set {index} should be resolvable by name"
            );
        }
        for (index, set) in BUILT_IN.iter().enumerate() {
            for other in BUILT_IN.iter().skip(index + 1) {
                assert_ne!(set.name(), other.name(), "names must be unique");
            }
        }
    }

    #[test]
    fn char_at_wraps_around() {
        let set = GlyphSet::default_set();
        let count = set.len();
        assert_eq!(set.char_at(0), set.char_at(count));
        assert_eq!(set.char_at(1), set.char_at(count + 1));
    }

    #[test]
    fn cycled_walks_every_set_then_returns_home() {
        let mut set = GlyphSet::default_set();
        for _ in 0..BUILT_IN.len() {
            set = set.cycled();
        }
        assert_eq!(set, GlyphSet::default_set());
    }

    #[test]
    fn by_name_is_case_sensitive_and_honest() {
        assert!(GlyphSet::by_name("bone").is_some());
        assert!(GlyphSet::by_name("Bone").is_none());
        assert!(GlyphSet::by_name("nope").is_none());
    }
}
