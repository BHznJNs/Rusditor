mod core;
mod file_saver;

pub use self::core::Component;
pub use file_saver::FileSaver;

use std::io;

use crossterm::event::KeyEvent;

use crate::editor::mode::EditorMode;

pub struct EditorComponentManager {
    pub file_saver: FileSaver,
}

impl EditorComponentManager {
    pub fn new() -> Self {
        Self {
            file_saver: FileSaver::new(),
        }
    }

    pub fn resolve(&mut self, current_mode: EditorMode, key: KeyEvent) -> io::Result<()> {
        match current_mode {
            EditorMode::Saving => &mut self.file_saver,
            _ => unreachable!(),
        }
        .key_resolve(key.code)?;
        return Ok(());
    }
}
