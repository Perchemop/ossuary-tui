//! Ossuary core — the headless heart of the Ossuary terminal art toy.
//!
//! Everything that decides *what* is drawn lives here: the glyph sets, the
//! hand-typed ASCII art pieces, the falling-character rain simulation and the
//! theme (settings) model with its on-disk persistence. Nothing in this crate
//! touches a terminal, a display or the network, so every workflow can be
//! exercised by `cargo test` in CI with no screen attached (guide §K.3, §K.13).
//!
//! ```
//! use ossuary_core::{ArtPack, Config, Engine};
//!
//! let mut engine = Engine::new(40, 12, Config::default(), ArtPack::built_in(), 42);
//! for _ in 0..10 {
//!     engine.step();
//! }
//! assert_eq!(engine.frame().width(), 40);
//! ```

mod art;
mod config;
mod engine;
mod error;
mod glyphs;
mod rng;

pub use art::{ArtPack, ArtPiece};
pub use config::Config;
pub use engine::{ArtState, Cell, Engine, Frame};
pub use error::CoreError;
pub use glyphs::GlyphSet;
pub use rng::Rng;

pub use config::{
    ART_INTERVAL_MAX, ART_INTERVAL_MIN, SPEED_MAX, SPEED_MIN, TRAIL_MAX, TRAIL_MIN, config_dir,
    default_config_path, log_dir, user_art_dir,
};

/// Application name, used in configuration paths and diagnostics.
pub const APP_NAME: &str = "ossuary";

/// The crate version, mirrored from `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
