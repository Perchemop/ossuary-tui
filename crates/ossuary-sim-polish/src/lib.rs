//! Simulation polish layer for Ossuary.
//!
//! This crate provides optional, composable enhancements on top of `ossuary-core`
//! without modifying the core engine. New behaviours are added as pure modules
//! that wrap or decorate the core types.

pub mod column_smoothing;
pub mod art_reveal_ease;

pub use column_smoothing::ColumnSmoothing;
pub use art_reveal_ease::ArtRevealEase;
