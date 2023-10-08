#![allow(clippy::needless_return)]

mod editor;
mod utils;

use std::{io, path::Path};

use clap::Parser;

use editor::Editor;

#[derive(Parser, Debug)]
#[command(name="Rusditor", version)]
struct Args {
    file_path: Option<String>,
    #[arg(short, long, long_help="Set accent color for this editor; Options: [red, blue, dark_red, dark_blue, dark_grey, dark_cyan, dark_yellow, dark_magenta]")]
    accent_color: Option<String>,
}

fn main() -> io::Result<()> {
    let mut editor = Editor::new();
    let args = Args::parse();
    if let Some(path) = args.file_path {
        let file_path = Path::new(&path);
        if file_path.exists() {
            editor.read_file(&path)?;
        }
    }
    if let Some(color) = args.accent_color {
        Editor::set_accent_color(&color);
    }

    if cfg!(debug_assertions) {
        // debug mode: show `io::Error` message
        editor.init()?;
        editor.cycle()?;
    } else {
        // release mode: exit directly
        editor.init().or_else(|_| editor.close())?;
        editor.cycle().or_else(|_| editor.close())?;
    }
    return Ok(());
}
