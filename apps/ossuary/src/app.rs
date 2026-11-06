//! The terminal shell: argument parsing, the event loop and the painter.

use std::io::{self, BufWriter, Stdout, Write};
use std::path::PathBuf;
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use ossuary_core::{
    ArtLibrary, Config, Engine, VERSION, default_config_path, log_dir, user_art_dir,
};

use crate::palette::color_for;

/// Milliseconds between simulation steps (roughly 24 frames per second).
const FRAME_MS: u64 = 42;

/// Embedded changelog for the in-app release-history overlay.
const RELEASE_NOTES: &str = include_str!("../../../CHANGELOG.md");

/// Embedded token/effort ledger shown alongside the changelog (guide §12.1).
const TOKEN_LEDGER: &str = include_str!("../../../docs/tokens.json");

/// The interactive application state.
struct App {
    engine: Engine,
    config: Config,
    config_path: Option<PathBuf>,
    paused: bool,
    quit: bool,
    show_help: bool,
    show_notes: bool,
    status: String,
}

impl App {
    fn new(config_path: Option<PathBuf>, seed: u64) -> Self {
        // A missing theme file means this is the first run: open the help
        // overlay as a welcome so the keys are discoverable immediately.
        let first_run = match &config_path {
            Some(path) => !path.exists(),
            None => false,
        };
        let loaded = config_path
            .as_deref()
            .map(Config::load_or_default)
            .unwrap_or_default();
        let library = user_art_dir()
            .map(|dir| ArtLibrary::built_in_with_user_art(&dir))
            .unwrap_or_else(ArtLibrary::built_in);

        let (cols, rows) = terminal::size().unwrap_or((100, 30));
        let width = (cols as usize).max(20);
        // The bottom row is reserved for the status line and the version.
        let height = (rows as usize).max(6).saturating_sub(1);

        let engine = Engine::new(width, height, loaded, library, seed);
        let config = engine.config().clone();
        Self {
            engine,
            config,
            config_path,
            paused: false,
            quit: false,
            show_help: first_run,
            show_notes: false,
            status: if first_run {
                "welcome to Ossuary — press ? for help".to_string()
            } else {
                "press ? for help".to_string()
            },
        }
    }

    fn apply_config(&mut self) {
        self.engine.set_config(self.config.clone());
    }

    fn note(&mut self, message: &str) {
        self.status = message.to_string();
    }

    fn on_resize(&mut self, cols: u16, rows: u16) {
        let width = (cols as usize).max(20);
        let height = (rows as usize).max(6).saturating_sub(1);
        self.engine.resize(width, height);
    }

    fn on_key(&mut self, key: KeyEvent) {
        if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
            return;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.quit = true;
            return;
        }

        // Overlays swallow their own dismissal keys.
        if self.show_notes {
            if matches!(
                key.code,
                KeyCode::Esc | KeyCode::Char('V') | KeyCode::Char('v') | KeyCode::Char('?')
            ) {
                self.show_notes = false;
            }
            return;
        }
        if self.show_help {
            if matches!(
                key.code,
                KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('h')
            ) {
                self.show_help = false;
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Char(' ') => {
                self.paused = !self.paused;
                self.note(if self.paused { "paused" } else { "resumed" });
            }
            KeyCode::Char('g') => {
                let next = self.engine.glyphs().cycled();
                self.config.glyph_set = next.name().to_string();
                self.apply_config();
                let name = next.name();
                self.note(&format!("glyph set: {name}"));
            }
            KeyCode::Char('n') => {
                self.engine.summon();
                let name = self
                    .engine
                    .current_piece()
                    .map_or("art", |piece| piece.name());
                self.note(&format!("summoning: {name}"));
            }
            KeyCode::Char('a') => {
                self.config.show_art = !self.config.show_art;
                self.apply_config();
                let state = if self.config.show_art { "on" } else { "off" };
                self.note(&format!("automatic art: {state}"));
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.config.speed_up();
                self.apply_config();
                let speed = self.config.speed;
                self.note(&format!("speed: {speed:.2}"));
            }
            KeyCode::Char('-') | KeyCode::Char('_') => {
                self.config.speed_down();
                self.apply_config();
                let speed = self.config.speed;
                self.note(&format!("speed: {speed:.2}"));
            }
            KeyCode::Char(']') => {
                self.config.denser();
                self.apply_config();
                let density = self.config.density;
                self.note(&format!("density: {density:.1}"));
            }
            KeyCode::Char('[') => {
                self.config.sparser();
                self.apply_config();
                let density = self.config.density;
                self.note(&format!("density: {density:.1}"));
            }
            KeyCode::Char('f') => {
                self.config.cycle_flicker();
                self.apply_config();
                let flicker = self.config.flicker;
                self.note(&format!("flicker: {flicker:.2}"));
            }
            KeyCode::Char('m') => {
                self.config.monochrome = !self.config.monochrome;
                self.apply_config();
                let palette = if self.config.monochrome {
                    "monochrome"
                } else {
                    "green"
                };
                self.note(&format!("palette: {palette}"));
            }
            KeyCode::Char('s') => self.save(),
            KeyCode::Char('r') => {
                self.config.cycle_art_mode();
                self.apply_config();
                let mode = self.config.art_mode.clone();
                self.note(&format!("art mode: {mode}"));
            }
            KeyCode::Char('d') => {
                self.config.color_drift = !self.config.color_drift;
                self.apply_config();
                let state = if self.config.color_drift { "on" } else { "off" };
                self.note(&format!("colour drift: {state}"));
            }
            KeyCode::Char('t') => {
                self.config.show_titles = !self.config.show_titles;
                self.apply_config();
                let state = if self.config.show_titles { "on" } else { "off" };
                self.note(&format!("art titles: {state}"));
            }
            KeyCode::Char('?') | KeyCode::Char('h') => self.show_help = true,
            KeyCode::Char('V') => self.show_notes = true,
            _ => {}
        }
    }

    fn save(&mut self) {
        match &self.config_path {
            Some(path) => match self.config.save_to(path) {
                Ok(()) => {
                    let shown = path.display().to_string();
                    self.note(&format!("saved theme to {shown}"));
                }
                Err(error) => self.note(&format!("could not save theme: {error}")),
            },
            None => self.note("no configuration directory available to save into"),
        }
    }

    fn draw(&self, out: &mut BufWriter<Stdout>) -> io::Result<()> {
        let frame = self.engine.frame();
        let width = frame.width();
        let height = frame.height();

        for y in 0..height {
            queue!(out, MoveTo(0, y as u16))?;
            let mut x = 0;
            while x < width {
                let intensity = frame.cell(x, y).intensity;
                let mut run = String::new();
                while x < width && frame.cell(x, y).intensity == intensity {
                    run.push(frame.cell(x, y).ch);
                    x += 1;
                }
                queue!(
                    out,
                    SetForegroundColor(color_for(intensity, self.config.monochrome)),
                    Print(run)
                )?;
            }
        }

        // Status line, then the version in the bottom-right corner (dim and
        // non-intrusive, per guide §9).
        let status_row = height as u16;
        queue!(
            out,
            MoveTo(0, status_row),
            SetForegroundColor(Color::DarkGrey),
            Print(pad_or_truncate(&self.status, width))
        )?;
        let version = format!("v{VERSION}");
        let version_x = width.saturating_sub(version.chars().count()) as u16;
        queue!(
            out,
            MoveTo(version_x, status_row),
            SetForegroundColor(Color::DarkGrey),
            Print(&version),
            ResetColor
        )?;

        if self.show_help {
            self.draw_help(out, width)?;
        }
        if self.show_notes {
            self.draw_notes(out, width, height as u16)?;
        }
        out.flush()
    }

    fn draw_help(&self, out: &mut BufWriter<Stdout>, width: usize) -> io::Result<()> {
        const LINES: [&str; 16] = [
            "Ossuary — keys",
            "",
            "space      pause / resume",
            "g          next glyph set",
            "n          summon an art piece",
            "a          toggle automatic art",
            "r          next art rotation mode",
            "+ / -      faster / slower",
            "] / [      denser / sparser",
            "f          cycle flicker",
            "m          monochrome / green",
            "d          toggle colour drift",
            "t          toggle art titles",
            "s          save theme",
            "V          changelog",
            "? or Esc   close this help   |   q  quit",
        ];
        for (index, line) in LINES.iter().enumerate() {
            let y = 1 + index as u16;
            queue!(
                out,
                MoveTo(2, y),
                SetForegroundColor(Color::Grey),
                Print(pad_or_truncate(line, width.saturating_sub(2)))
            )?;
        }
        queue!(out, ResetColor)?;
        Ok(())
    }

    fn draw_notes(&self, out: &mut BufWriter<Stdout>, width: usize, height: u16) -> io::Result<()> {
        let header = format!("Ossuary {VERSION} — changelog (V or Esc to close)");
        queue!(
            out,
            MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print(pad_or_truncate(&header, width))
        )?;

        let available = height.saturating_sub(2) as usize;
        for (index, line) in RELEASE_NOTES.lines().take(available).enumerate() {
            queue!(
                out,
                MoveTo(0, 1 + index as u16),
                SetForegroundColor(Color::Grey),
                Print(pad_or_truncate(line, width))
            )?;
        }

        let tokens = format!("token metrics: {}", compact(TOKEN_LEDGER));
        queue!(
            out,
            MoveTo(0, height),
            SetForegroundColor(Color::DarkGrey),
            Print(pad_or_truncate(&tokens, width)),
            ResetColor
        )?;
        Ok(())
    }
}

/// Pad with spaces or truncate to exactly `width` display columns.
fn pad_or_truncate(text: &str, width: usize) -> String {
    let mut out = String::with_capacity(width);
    for ch in text.chars().take(width) {
        out.push(ch);
    }
    while out.chars().count() < width {
        out.push(' ');
    }
    out
}

/// Collapse all whitespace runs in `text` into single spaces.
fn compact(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Parse `--help`, `--version`, `--config`, `--seed`; run the app.
pub fn run(args: Vec<String>) -> std::process::ExitCode {
    let mut config_path = default_config_path();
    let mut seed = seed_from_clock();

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-h" | "--help" => {
                print_help();
                return std::process::ExitCode::SUCCESS;
            }
            "-V" | "--version" => {
                println!("ossuary {VERSION}");
                return std::process::ExitCode::SUCCESS;
            }
            "--config" => {
                if let Some(path) = args.get(index + 1) {
                    config_path = Some(PathBuf::from(path));
                    index += 1;
                }
            }
            "--seed" => {
                if let Some(value) = args.get(index + 1).and_then(|raw| raw.parse::<u64>().ok()) {
                    seed = value;
                    index += 1;
                }
            }
            _ => {}
        }
        index += 1;
    }

    // A panic must leave a trace, never a silent exit (guide §6.1).
    if let Some(dir) = log_dir() {
        crash_report::install_panic_hook("ossuary", VERSION, &dir);
    }

    let mut app = App::new(config_path, seed);
    match run_loop(&mut app) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ossuary: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run_loop(app: &mut App) -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let result = event_loop(app);

    // Always restore the terminal, even when the loop returned an error.
    execute!(stdout, LeaveAlternateScreen, Show, ResetColor)?;
    terminal::disable_raw_mode()?;
    result
}

fn event_loop(app: &mut App) -> io::Result<()> {
    let mut out = BufWriter::new(io::stdout());
    while !app.quit {
        if event::poll(Duration::from_millis(FRAME_MS))? {
            loop {
                match event::read()? {
                    Event::Key(key) => app.on_key(key),
                    Event::Resize(cols, rows) => app.on_resize(cols, rows),
                    _ => {}
                }
                if !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }
        if !app.paused {
            app.engine.step();
        }
        app.draw(&mut out)?;
    }
    Ok(())
}

/// A time-based seed, so each launch differs.
fn seed_from_clock() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0x0DDB_1A5E_5BAD_5EED, |elapsed| elapsed.as_nanos() as u64)
}

fn print_help() {
    println!(
        "Ossuary {VERSION} — a terminal ASCII-art rain toy.\n\
         \n\
         Usage: ossuary [--config <path>] [--seed <n>]\n\
         \n\
         Options:\n\
           --config <path>   theme file to load and save (default: the per-user config dir)\n\
           --seed <n>        fixed random seed (default: time-based)\n\
           -V, --version     print the version and exit\n\
           -h, --help        print this help and exit\n\
         \n\
         Keys: space pause · g glyphs · n art · a auto-art · r rotation ·\n\
               +/- speed · [/] density · f flicker · m palette ·\n\
               d drift · t titles · s save · V changelog · ? help · q quit"
    );
}
