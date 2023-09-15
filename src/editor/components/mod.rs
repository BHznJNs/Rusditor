mod core;
mod file_saver;
mod finder;
mod history;
mod positioner;

pub use self::core::Component;
pub use file_saver::FileSaver;
pub use finder::Finder;
pub use positioner::Positioner;

use std::io;

use crossterm::event::KeyEvent;

use super::core::EditorMode;

pub struct EditorComponentManager {
    pub file_saver: FileSaver,
    pub finder: Finder,
    pub positioner: Positioner,
}

impl EditorComponentManager {
    pub fn new() -> Self {
        Self {
            file_saver: FileSaver::new(),
            finder: Finder::new(),
            positioner: Positioner::new(),
        }
    }

    pub fn resolve(&mut self, current_mode: EditorMode, key: KeyEvent) -> io::Result<()> {
        match current_mode {
            EditorMode::Saving => self.file_saver.key_resolve(key.code)?,
            EditorMode::Finding => self.finder.key_resolve(key.code)?,
            EditorMode::Positioning => self.positioner.key_resolve(key.code)?,
            _ => unreachable!(),
        }
        return Ok(());
    }
}
