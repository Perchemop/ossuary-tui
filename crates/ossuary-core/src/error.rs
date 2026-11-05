use std::path::PathBuf;

/// Errors produced by Ossuary's core.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    /// A filesystem operation failed.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// A theme file could not be parsed.
    #[error("could not parse theme: {0}")]
    Parse(#[from] serde_json::Error),

    /// No usable configuration directory exists for this user.
    #[error("no configuration directory could be determined")]
    NoConfigDir,

    /// A glyph-set name was not recognised.
    #[error("unknown glyph set: {0}")]
    UnknownGlyphSet(String),

    /// A path was expected to name a file but did not.
    #[error("invalid path: {}", .0.display())]
    InvalidPath(PathBuf),
}
