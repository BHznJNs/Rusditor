mod core;
mod file_saver;
mod file_opener;
mod finder;
mod positioner;
mod replacer;
mod helper;

pub use self::core::LineComponent;
pub use file_saver::FileSaver;
pub use file_opener::FileOpener;
pub use finder::Finder;
pub use positioner::Positioner;
pub use replacer::Replacer;
pub use helper::Helper;

use std::io;

use crossterm::event::KeyEvent;

use super::core::EditorState;

pub struct EditorComponentManager {
    pub use_line_component: bool,
    pub use_screen_component: bool,

    // line components
    pub file_saver: FileSaver,
    pub file_opener: FileOpener,
    pub positioner: Positioner,
    pub finder: Finder,
    pub replacer: Replacer,

    // screen components
    pub helper: Helper,
}

impl EditorComponentManager {
    pub fn new() -> Self {
        Self {
            use_line_component: false,
            use_screen_component: false,

            file_saver: FileSaver::new(),
            file_opener: FileOpener::new(),
            positioner: Positioner::new(),
            finder: Finder::new(),
            replacer: Replacer::new(),

            helper: Helper::new(),
        }
    }

    pub fn resolve(&mut self, current_state: EditorState, key: KeyEvent) -> io::Result<()> {
        match current_state {
            EditorState::Saving => self.file_saver.key_resolve(key)?,
            EditorState::Opening => self.file_opener.key_resolve(key)?,
            EditorState::Positioning => self.positioner.key_resolve(key)?,
            EditorState::Finding => self.finder.key_resolve(key)?,
            EditorState::Replacing => self.replacer.key_resolve(key)?,

            EditorState::ReadingHelpMsg => self.helper.key_resolve(key)?,
            _ => unreachable!(),
        }
        return Ok(());
    }
}
