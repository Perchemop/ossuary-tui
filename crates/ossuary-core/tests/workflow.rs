//! Headless workflow tests for Ossuary's core (guide §K.5, §M.2).
//!
//! These exercise the real end-to-end paths — theme persistence, the rain
//! simulation, determinism, the art-piece lifecycle and user art packs — with
//! no terminal, no display and no network.

use ossuary_core::{ArtPack, ArtState, Cell, Config, Engine};

/// Count the cells whose brightness band equals `intensity`.
fn count_intensity(engine: &Engine, intensity: u8) -> usize {
    let frame = engine.frame();
    let mut total = 0;
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            if frame.cell(x, y).intensity == intensity {
                total += 1;
            }
        }
    }
    total
}

#[test]
fn workflow_theme_save_load_roundtrip() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("theme.json");

    let config = Config {
        glyph_set: "letters".to_string(),
        speed: 1.25,
        density: 0.4,
        trail: 42,
        flicker: 0.5,
        show_art: false,
        art_interval: 500,
        monochrome: false,
    };
    config.save_to(&path).expect("save theme");

    let loaded = Config::load_from(&path).expect("load theme");
    assert_eq!(loaded, config);
}

#[test]
fn workflow_rain_lights_cells_from_the_active_glyph_set() {
    let mut engine = Engine::new(40, 14, Config::default(), ArtPack::empty(), 7);
    for _ in 0..40 {
        engine.step();
    }

    let glyphs = engine.glyphs();
    let allowed: Vec<char> = glyphs.chars().chars().collect();
    let frame = engine.frame();
    let mut lit = 0;
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let cell = frame.cell(x, y);
            if !cell.is_empty() {
                lit += 1;
                assert!(
                    allowed.contains(&cell.ch),
                    "glyph {:?} is not in set {}",
                    cell.ch,
                    glyphs.name()
                );
            }
        }
    }
    assert!(lit > 0, "the rain should light at least one cell");
}

#[test]
fn workflow_same_seed_is_deterministic_and_different_seeds_diverge() {
    let config = Config::default();
    let mut a = Engine::new(60, 18, config.clone(), ArtPack::empty(), 12345);
    let mut b = Engine::new(60, 18, config.clone(), ArtPack::empty(), 12345);
    let mut other = Engine::new(60, 18, config, ArtPack::empty(), 999);

    for _ in 0..120 {
        a.step();
        b.step();
    }
    for _ in 0..120 {
        other.step();
    }

    assert_eq!(
        a.frame(),
        b.frame(),
        "same seed must reproduce the same frame"
    );
    assert_ne!(a.frame(), other.frame(), "different seeds should differ");
}

#[test]
fn workflow_art_piece_is_typed_completed_and_released() {
    // Control the art manually rather than on a timer.
    let config = Config {
        show_art: false,
        ..Config::default()
    };
    let mut engine = Engine::new(40, 14, config, ArtPack::built_in(), 5);

    engine.summon();
    assert_eq!(engine.art_state(), ArtState::Typing);

    let mut steps = 0;
    while engine.art_state() != ArtState::Holding && steps < 2_000 {
        engine.step();
        steps += 1;
    }
    assert_eq!(engine.art_state(), ArtState::Holding);

    let piece_ink = engine
        .current_piece()
        .expect("a piece is on screen")
        .ink_count();
    assert!(piece_ink > 0);
    // A fully drawn piece contributes bright cells on top of the rain heads.
    assert!(count_intensity(&engine, 3) >= piece_ink);

    while engine.art_state() != ArtState::Idle && steps < 8_000 {
        engine.step();
        steps += 1;
    }
    assert_eq!(engine.art_state(), ArtState::Idle);
    assert_eq!(engine.tick(), steps);
}

#[test]
fn workflow_user_art_pack_extends_the_built_in_pack() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("arrow.txt"), " -->\n  |\n").expect("write art");

    let user = ArtPack::load_dir(dir.path()).expect("load user pack");
    assert_eq!(user.len(), 1);
    assert_eq!(user.piece(0).expect("piece").name(), "arrow");

    let combined = ArtPack::built_in_with_user_art(dir.path());
    assert!(combined.len() > user.len());
    assert!(combined.index_of("arrow").is_some());
    assert!(combined.index_of("skull").is_some());

    // An empty frame is exactly that: all cells empty, correct size.
    let frame = ossuary_core::Frame::new(4, 3);
    assert_eq!(frame.width(), 4);
    assert_eq!(frame.height(), 3);
    assert_eq!(frame.cell(2, 2), Cell::EMPTY);
}
