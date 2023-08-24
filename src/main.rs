mod editor;
mod utils;

use editor::Editor;
use std::io;

fn main() -> io::Result<()> {
    let mut editor = Editor::new();
    editor.init()?;
    editor.cycle()?;
    return Ok(());
}
