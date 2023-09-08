mod editor;
mod utils;

use editor::Editor;
use std::{env, io, path::Path};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut editor = Editor::new();

    if args.len() > 1 {
        let file_path_str = &args[1];
        let file_path = Path::new(file_path_str);
        if file_path.exists() {
            editor.read_file(file_path_str)?;
        }
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
