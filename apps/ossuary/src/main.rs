//! Ossuary — a terminal ASCII-art rain toy.
//!
//! The binary is a thin shell: it owns the terminal, collects keys and paints
//! the frames that [`ossuary_core::Engine`] produces. All simulation logic lives
//! in the core crate, so it is fully testable without a terminal (guide §K.3).

mod app;
mod palette;

use std::process::ExitCode;

fn main() -> ExitCode {
    app::run(std::env::args().skip(1).collect())
}
