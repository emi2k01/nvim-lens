#![allow(dead_code)]

#[macro_use]
extern crate tracing;

use std::{
    io::Read,
    io::Write,
    ops::DerefMut,
    path::Path,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

use alacritty_terminal::{
    index::{Column, Line},
    term::cell::Cell,
    Grid,
};
use include_dir::{include_dir, Dir};
use tracing_subscriber::EnvFilter;

mod args;
mod color;
mod config;
mod nvim;
mod vte;

static SAMPLES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/samples");
static SAMPLES_EXTRACTION_PATH: &str = "/tmp/_$:$_samples";

const COLUMNS: usize = 110;
const LINES: usize = 40;

#[derive(Debug)]
enum Action {
    NvimCmd(String),
    Snapshot { title: String },
}

#[derive(Debug)]
struct Span {
    bg: String,
    fg: String,
    text: String,
}

fn actions(colorscheme: &str) -> Vec<Action> {
    vec![
        Action::NvimCmd(format!(":tabnew<CR>")),
        Action::NvimCmd(format!(":-tabclose<CR>")),
        Action::NvimCmd(format!(":colorscheme {colorscheme}<CR>")),
        Action::NvimCmd(format!(":e sample.rs<CR>")),
        Action::Snapshot {
            title: "Rust".to_string(),
        },
        Action::NvimCmd(format!(":e sample.tsx<CR>")),
        Action::Snapshot {
            title: "Typescript".to_string(),
        },
        Action::NvimCmd(format!(":e sample_diff_0.rs | diffthis<CR>")),
        Action::NvimCmd(format!(":vert new sample_diff_1.rs | diffthis<CR>")),
        Action::Snapshot {
            title: "Diff".to_string(),
        },
        Action::NvimCmd(format!(":Telescope find_files<CR>")),
        Action::Snapshot {
            title: "Telescope".to_string(),
        },
        Action::NvimCmd(format!(":q<CR>")),
    ]
}

fn extract_samples() {
    if Path::new(SAMPLES_EXTRACTION_PATH).exists() {
        std::fs::remove_dir_all(SAMPLES_EXTRACTION_PATH).unwrap();
    }
    std::fs::create_dir_all(SAMPLES_EXTRACTION_PATH).unwrap();
    SAMPLES_DIR.extract(SAMPLES_EXTRACTION_PATH).unwrap();
}

fn export_snapshot(out_dir: &Path, grid: &Grid<Cell>, title: &str) {
    std::fs::create_dir_all(out_dir).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(std::path::Path::new(out_dir).join(&format!("{title}.html")))
        .unwrap();
    let mut lines: Vec<Vec<Span>> = Vec::new();
    for line in 0..LINES {
        let mut line_spans: Vec<Span> = Vec::new();
        let row = &grid[Line(line as i32)];
        for col in 0..COLUMNS {
            let cell = &row[Column(col)];
            let ch = match cell.c {
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '"' => "&quot;".to_string(),
                ch => ch.to_string(),
            };
            let bg = color::to_string(cell.bg);
            let fg = color::to_string(cell.fg);
            let merge_with_last_span = line_spans
                .last_mut()
                .map_or(false, |span| span.bg == bg && span.fg == fg);
            if merge_with_last_span {
                line_spans.last_mut().unwrap().text.push_str(&ch);
            } else {
                line_spans.push(Span { bg, fg, text: ch });
            }
        }
        lines.push(line_spans);
    }
    write!(file, "<pre class='term-nvim-screen'><code>").unwrap();
    for line in &lines {
        for span in line {
            let Span { bg, fg, text } = span;
            write!(
                file,
                "<span style='background-color:{bg};color:{fg};'>{text}</span>"
            )
            .unwrap();
        }
        write!(file, "\n").unwrap();
    }
    write!(file, "</code></pre>").unwrap();
}

fn main() {
    tracing_subscriber::FmtSubscriber::builder()
        .with_ansi(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    dotenvy::dotenv().unwrap();
    color_eyre::install().unwrap();
    let args = args::parse();
    let config = config::load(Path::new(&args.config));
    nvim::setup(&config.plugins);
    extract_samples();
    let term = Arc::new(Mutex::new(vte::term()));
    let mut vte_processor = vte::processor();
    let mut nvim_process = nvim::spawn();
    // wait for nvim to start
    std::thread::sleep(Duration::from_millis(500));
    let nvim_stdout = nvim_process.stdout.take().unwrap();
    let read_byte_sync = Arc::new((Mutex::new(false), Condvar::new()));

    // Read neovim's stdout on another thread.
    std::thread::spawn({
        let term = term.clone();
        let read_byte_sync = Arc::clone(&read_byte_sync);

        move || {
            let (read_byte, read_byte_cvar) = &*read_byte_sync;
            for byte in nvim_stdout.bytes() {
                let mut term = term.lock().unwrap();
                let byte = byte.unwrap();
                vte_processor.advance(term.deref_mut(), byte);
                // Send a notification so that the main thread knows
                // that neovim's hasn't finished rendering yet.
                *read_byte.lock().unwrap() = true;
                read_byte_cvar.notify_one();
            }
        }
    });
    let read_byte_sync = Arc::clone(&read_byte_sync);

    for plugin in &config.plugins {
        for colorscheme in &plugin.colorschemes {
            for action in actions(&colorscheme) {
                let out_dir = Path::new(&args.out_dir).join(&plugin.id).join(colorscheme);
                match action {
                    Action::NvimCmd(cmd) => {
                        nvim::send_command(&cmd);
                    }
                    Action::Snapshot { title } => {
                        loop {
                            std::io::stdout().flush().unwrap();
                            // Wait for neovim's stdout or for N millis since we read neovim's stdout
                            let is_stdout_exhausted = {
                                let (read_byte, read_byte_cvar) = &*read_byte_sync;
                                let guard = read_byte.lock().unwrap();
                                let mut res = read_byte_cvar
                                    .wait_timeout_while(
                                        guard,
                                        Duration::from_millis(100),
                                        |&mut read_byte| !read_byte,
                                    )
                                    .unwrap();
                                *res.0 = false;
                                res.1.timed_out()
                            };

                            // We have read all of neovim's stdout so far so we can proceed executing the actions
                            if is_stdout_exhausted {
                                let grid = {
                                    let term = term.lock().unwrap();
                                    term.grid().clone()
                                };
                                export_snapshot(&out_dir, &grid, &title);
                                break;
                            } else {
                                // if stdout wasn't exhausted then try again
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
    // Wait for neovim to process last commands
    std::thread::sleep(Duration::from_millis(100));
    // kill the process
    nvim_process.kill().unwrap();
}
