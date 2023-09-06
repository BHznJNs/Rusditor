mod core;
mod file_saver;

pub use file_saver::FileSaver;
pub use self::core::Component;

use std::io;

use crossterm::event::KeyEvent;

use crate::editor::mode::EditorMode;


pub struct Components {
    pub file_saver: FileSaver,
}

impl Components {
    pub fn new() -> Self {
        Self {
            file_saver: FileSaver::new(),
        }
    }

    pub fn resolve(&mut self, current_mode: EditorMode, key: KeyEvent) -> io::Result<()> {
        match current_mode {
            EditorMode::Saving => &mut self.file_saver,
            _ => unreachable!()
        }.key_resolve(key.code)?;
        return Ok(());
    }
}