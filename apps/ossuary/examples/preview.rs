//! Generate the preview images used in the README and docs.
//!
//! Usage: `cargo run -p ossuary --release --example preview -- <output-dir>`
//!
//! Images are built from the deterministic engine, so a given seed always
//! produces the same picture (guide §M.5). This runs headlessly; it never opens
//! a terminal.

use std::io;
use std::path::Path;
use std::process::ExitCode;

use ossuary_core::{ArtLibrary, ArtState, Config, Engine, render_svg};

/// A wide grid so the pictures have room to breathe.
const COLS: usize = 96;
const ROWS: usize = 30;

fn main() -> ExitCode {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "docs/images".to_string());
    match run(&dir) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("preview: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(dir: &str) -> io::Result<()> {
    let out = Path::new(dir);
    std::fs::create_dir_all(out)?;

    // The rain on its own.
    let mut engine = Engine::new(COLS, ROWS, Config::default(), ArtLibrary::built_in(), 7);
    for _ in 0..48 {
        engine.step();
    }
    std::fs::write(
        out.join("ossuary-rain.svg"),
        render_svg(&engine.frame(), "Ossuary — falling characters"),
    )?;

    // A skull fully drawn, with its title shown.
    let config = Config {
        show_art: false,
        show_titles: true,
        art_mode: "single".to_string(),
        art_piece: "skull".to_string(),
        ..Config::default()
    };
    let mut engine = Engine::new(COLS, ROWS, config, ArtLibrary::built_in(), 3);
    engine.summon();
    for _ in 0..2_000 {
        engine.step();
        if engine.art_state() == ArtState::Holding {
            break;
        }
    }
    std::fs::write(
        out.join("ossuary-art.svg"),
        render_svg(&engine.frame(), "Ossuary — a hand-typed piece"),
    )?;

    Ok(())
}
