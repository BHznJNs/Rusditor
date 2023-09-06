mod editor;
mod utils;

use editor::Editor;
use std::{io, env};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut editor = Editor::new();

    if args.len() > 1 {
        editor.read_file(&args[1])?;
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
