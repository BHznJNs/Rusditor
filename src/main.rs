mod editor;
mod utils;

use std::io;
use editor::Editor;

fn main() -> io::Result<()> {
    let mut editor = Editor::new();
    editor.init()?;
    editor.cycle()?;
    return Ok(());
}
