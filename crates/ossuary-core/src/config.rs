//! The persistable settings model ("theme") and the paths it lives at.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// Lower bound for [`Config::speed`] (rows advanced per step).
pub const SPEED_MIN: f32 = 0.05;
/// Upper bound for [`Config::speed`].
pub const SPEED_MAX: f32 = 4.0;
/// Lower bound for [`Config::trail`] (maximum column length).
pub const TRAIL_MIN: usize = 2;
/// Upper bound for [`Config::trail`].
pub const TRAIL_MAX: usize = 80;
/// Lower bound for [`Config::art_interval`] (steps between art pieces).
pub const ART_INTERVAL_MIN: u32 = 40;
/// Upper bound for [`Config::art_interval`].
pub const ART_INTERVAL_MAX: u32 = 6000;

/// The complete, persistable set of user settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Name of the active glyph set (see [`crate::GlyphSet`]).
    pub glyph_set: String,
    /// Rows advanced per simulation step.
    pub speed: f32,
    /// Fraction of columns that are active (0.0–1.0).
    pub density: f32,
    /// Maximum trail length, in rows.
    pub trail: usize,
    /// Probability per step that an active column swaps a character.
    pub flicker: f32,
    /// Whether the rain periodically summons complete art pieces.
    pub show_art: bool,
    /// Steps between automatically summoned art pieces.
    pub art_interval: u32,
    /// Monochrome (black & white) rather than the retro-green palette.
    pub monochrome: bool,
    /// Art rotation mode: `"sequential"`, `"random"` or `"single"`.
    pub art_mode: String,
    /// Piece to keep showing when `art_mode` is `"single"`.
    pub art_piece: String,
    /// Per-column brightness drift within the monochrome ramp.
    pub color_drift: bool,
    /// Draw the piece's name beneath it while it is on screen.
    pub show_titles: bool,
    /// Whether the polish layer is active, smoothing frame-to-frame glyph flicker
    /// and easing art reveal timing. Default `true` — a deterministic, reversible
    /// improvement with no new dependencies or dependencies on any external state.
    /// Visible in the help overlay so a user can turn it off.
    #[serde(default = "polish_true")]
    pub polish: bool,
    /// The strength of the polish EMA blend, in `0.0..=1.0`. Higher values give
    /// more smoothing but may increase per-frame cost. Default `0.5`; clamped if
    /// out of range. Round-trips through save/load; garbage / NaN values fall back
    /// to the default instead of failing startup.
    #[serde(default = "polish_strength_half")]
    pub polish_strength: f32,
    /// Visible in the help overlay so a user can turn it off.
    #[serde(default = "polish_true")]
    pub polish: bool,
    /// The strength of the polish EMA blend, in `0.0..=1.0`. Higher values give
    /// more smoothing but may increase per-frame cost. Default `0.5`; clamped if
    /// out of range. Round-trips through save/load; garbage / NaN values fall back
    /// to the default instead of failing startup.
    #[serde(default = "polish_strength_half")]
    pub polish_strength: f32,
}

/// The valid values of [`Config::art_mode`].
pub const ART_MODES: [&str; 3] = ["sequential", "random", "single"];

impl Default for Config {
    fn default() -> Self {
        Self {
            glyph_set: "bone".to_string(),
            speed: 0.5,
            density: 0.85,
            trail: 18,
            flicker: 0.15,
            show_art: true,
            art_interval: 320,
            monochrome: true,
            art_mode: "sequential".to_string(),
            art_piece: String::new(),
            color_drift: false,
            show_titles: false,
            show_phrases: true,
        }
    }
}

impl Config {
    /// Clamp every field into its valid range.
    #[must_use]
    pub fn sanitized(mut self) -> Self {
        self.speed = self.speed.clamp(SPEED_MIN, SPEED_MAX);
        self.density = self.density.clamp(0.0, 1.0);
        self.trail = self.trail.clamp(TRAIL_MIN, TRAIL_MAX);
        self.flicker = self.flicker.clamp(0.0, 1.0);
        self.art_interval = self.art_interval.clamp(ART_INTERVAL_MIN, ART_INTERVAL_MAX);
        if !ART_MODES.contains(&self.art_mode.as_str()) {
            self.art_mode = ART_MODES[0].to_string();
        }
        self
    }

    /// Cycle the art rotation mode: sequential → random → single → sequential.
    pub fn cycle_art_mode(&mut self) {
        let index = ART_MODES
            .iter()
            .position(|mode| *mode == self.art_mode)
            .unwrap_or(0);
        self.art_mode = ART_MODES[(index + 1) % ART_MODES.len()].to_string();
    }

    /// Nudge the speed up one step.
    pub fn speed_up(&mut self) {
        self.speed = (self.speed * 1.5).clamp(SPEED_MIN, SPEED_MAX);
    }

    /// Nudge the speed down one step.
    pub fn speed_down(&mut self) {
        self.speed = (self.speed / 1.5).clamp(SPEED_MIN, SPEED_MAX);
    }

    /// Increase the density by one tenth.
    pub fn denser(&mut self) {
        let steps = (self.density * 10.0).round() as i32;
        self.density = ((steps + 1) as f32 / 10.0).clamp(0.0, 1.0);
    }

    /// Decrease the density by one tenth.
    pub fn sparser(&mut self) {
        let steps = (self.density * 10.0).round() as i32;
        self.density = ((steps - 1).max(0) as f32 / 10.0).clamp(0.0, 1.0);
    }

    /// Cycle flicker through a small ladder: off → low → medium → high.
    pub fn cycle_flicker(&mut self) {
        let ladder = [0.0_f32, 0.1, 0.25, 0.5];
        self.flicker = ladder
            .iter()
            .position(|value| (*value - self.flicker).abs() < 1e-3)
            .map(|index| ladder[(index + 1) % ladder.len()])
            .unwrap_or(0.1);
    }

    /// Serialise as pretty JSON into `path`, creating parent directories.
    pub fn save_to(&self, path: &Path) -> Result<(), CoreError> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load and sanitise a theme from `path`.
    pub fn load_from(path: &Path) -> Result<Self, CoreError> {
        let text = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&text)?;
        Ok(config.sanitized())
    }

    /// Load a theme, falling back to defaults when the file is absent or
    /// unreadable — a broken theme must never stop the app from starting.
    pub fn load_or_default(path: &Path) -> Self {
        Self::load_from(path).unwrap_or_default()
    }
}

/// The per-user configuration directory, if one can be determined.
pub fn config_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("OSSUARY_CONFIG_DIR") {
        return Some(PathBuf::from(dir));
    }
    if let Some(dir) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(dir).join("ossuary"));
    }
    if let Some(home) = std::env::var_os("HOME") {
        return Some(PathBuf::from(home).join(".config").join("ossuary"));
    }
    if let Some(appdata) = std::env::var_os("APPDATA") {
        return Some(PathBuf::from(appdata).join("ossuary"));
    }
    None
}

/// The default theme file path.
pub fn default_config_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("theme.json"))
}

/// The default directory for user-supplied art pieces.
pub fn user_art_dir() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("art"))
}

/// The default crash-log directory.
pub fn log_dir() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("logs"))
}

/// Default-value helper for `Config::polish` — `true`.
///
/// Used by `#[serde(default = "polish_true")]` so the field is `true` when no
/// theme file is present, and old theme files (without this field) also default
/// to `true` rather than `false`.
fn polish_true() -> bool {
    true
}

/// Default-value helper for `Config::polish_strength` — `0.5`.
///
/// Used by `#[serde(default = "polish_strength_half")]` so the field is `0.5`
/// when no theme file is present, and old theme files (without this field) also
/// default to `0.5` rather than `0.0` or failing to parse.
fn polish_strength_half() -> f32 {
    0.5
}

/// The default crash-log directory.
pub fn log_dir() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join("logs"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_already_sanitised() {
        let default = Config::default();
        assert_eq!(default.clone().sanitized(), default);
    }

    #[test]
    fn sanitize_clamps_every_range() {
        let wild = Config {
            speed: 100.0,
            density: 5.0,
            trail: 0,
            flicker: -1.0,
            art_interval: 1,
            ..Config::default()
        }
        .sanitized();
        assert_eq!(wild.speed, SPEED_MAX);
        assert_eq!(wild.density, 1.0);
        assert_eq!(wild.trail, TRAIL_MIN);
        assert_eq!(wild.flicker, 0.0);
        assert_eq!(wild.art_interval, ART_INTERVAL_MIN);
    }

    #[test]
    fn speed_up_and_down_are_bounded_and_reversible() {
        let mut config = Config::default();
        let original = config.speed;
        config.speed_up();
        assert!(config.speed > original);
        config.speed_down();
        assert!((config.speed - original).abs() < 1e-4);
        for _ in 0..40 {
            config.speed_up();
        }
        assert_eq!(config.speed, SPEED_MAX);
        for _ in 0..80 {
            config.speed_down();
        }
        assert_eq!(config.speed, SPEED_MIN);
    }

    #[test]
    fn density_moves_by_tenths_and_stays_in_range() {
        let mut config = Config {
            density: 0.5,
            ..Config::default()
        };
        config.denser();
        assert!((config.density - 0.6).abs() < 1e-4);
        config.sparser();
        config.sparser();
        assert!((config.density - 0.4).abs() < 1e-4);
        for _ in 0..20 {
            config.denser();
        }
        assert_eq!(config.density, 1.0);
        for _ in 0..30 {
            config.sparser();
        }
        assert_eq!(config.density, 0.0);
    }

    #[test]
    fn flicker_ladder_cycles_through_all_steps() {
        let mut config = Config {
            flicker: 0.0,
            ..Config::default()
        };
        let mut seen = vec![config.flicker];
        for _ in 0..3 {
            config.cycle_flicker();
            seen.push(config.flicker);
        }
        assert_eq!(seen, vec![0.0, 0.1, 0.25, 0.5]);
        config.cycle_flicker();
        assert_eq!(config.flicker, 0.0);
    }

    #[test]
    fn theme_round_trips_through_a_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("nested").join("theme.json");
        let config = Config {
            glyph_set: "shade".to_string(),
            speed: 1.25,
            ..Config::default()
        };
        config.save_to(&path).expect("save");
        let loaded = Config::load_from(&path).expect("load");
        assert_eq!(loaded, config);
    }

    #[test]
    fn missing_theme_falls_back_to_default() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("absent.json");
        assert_eq!(Config::load_or_default(&path), Config::default());
    }

    #[test]
    fn partial_json_fills_missing_fields_from_default() {
        let config: Config = serde_json::from_str("{\"speed\": 2.0}").expect("parse");
        assert_eq!(config.speed, 2.0);
        assert_eq!(config.glyph_set, Config::default().glyph_set);
    }

    #[test]
    fn art_mode_cycles_and_unknown_values_reset() {
        let mut config = Config::default();
        assert_eq!(config.art_mode, "sequential");
        config.cycle_art_mode();
        assert_eq!(config.art_mode, "random");
        config.cycle_art_mode();
        assert_eq!(config.art_mode, "single");
        config.cycle_art_mode();
        assert_eq!(config.art_mode, "sequential");

        let bad = Config {
            art_mode: "sideways".to_string(),
            ..Config::default()
        }
        .sanitized();
        assert_eq!(bad.art_mode, "sequential");
    }
}

    #[test]
        assert!((loaded4.polish_strength - 0.5).abs() < 1e-4);

        let path5 = dir.path().join("clamp2.json");
        std::fs::write(&path5, r#"{"polish_strength": 2.0}"#).expect("write");
        let loaded5 = Config::load_from(&path5).expect("load");
        assert!((loaded5.polish_strength - 0.5).abs() < 1e-4);
    }
}

    #[test]
    fn polish_fields_clamp_and_default() {
        // Default values when no theme file is present.
        let default = Config::default();
        assert_eq!(default.polish, true);
        assert!((default.polish_strength - 0.5).abs() < 1e-4);

        // Old theme file (no polish fields) loads with defaults.
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("old.json");
        std::fs::write(&path, r#"{"glyph_set": "bone", "speed": 0.5}"#).expect("write");
        let loaded = Config::load_or_default(&path);
        assert_eq!(loaded.polish, true);
        assert!((loaded.polish_strength - 0.5).abs() < 1e-4);

        // New theme file (with polish fields) loads with those values.
        let path2 = dir.path().join("new.json");
        std::fs::write(&path2, r#"{"glyph_set": "bone", "speed": 0.5, "polish": true, "polish_strength": 0.75}"#).expect("write");
        let loaded2 = Config::load_from(&path2).expect("load");
        assert_eq!(loaded2.polish, true);
        assert!((loaded2.polish_strength - 0.75).abs() < 1e-4);

        // Garbage / NaN polish_strength falls back to default.
        let path3 = dir.path().join("bad.json");
        std::fs::write(&path3, r#"{"polish_strength": 5.0}"#).expect("write");
        let loaded3 = Config::load_from(&path3).expect("load");
        assert_eq!(loaded3.polish_strength, 0.5);

        // Clamp out-of-range values.
        let path4 = dir.path().join("clamp.json");
        std::fs::write(&path4, r#"{"polish_strength": -0.3}"#).expect("write");
        let loaded4 = Config::load_from(&path4).expect("load");
        assert!((loaded4.polish_strength - 0.5).abs() < 1e-4);

        let path5 = dir.path().join("clamp2.json");
        std::fs::write(&path5, r#"{"polish_strength": 2.0}"#).expect("write");
        let loaded5 = Config::load_from(&path5).expect("load");
        assert!((loaded5.polish_strength - 0.5).abs() < 1e-4);
    }

}
