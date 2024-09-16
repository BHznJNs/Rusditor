use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::text_area::TextArea;

use super::core::{LineComponent, LineComponentController};

pub struct FileOpener {
    comp: LineComponentController,
}

impl FileOpener {
    pub fn new() -> Self {
        Self {
            comp: Self::init_controller(),
        }
    }

    #[inline]
    pub fn get_file_path(&mut self) -> &str {
        self.comp.text_area.content()
    }

    #[inline]
    pub fn is_open_file_callback_key(key: KeyEvent) -> bool {
        key.modifiers == KeyModifiers::NONE && key.code == KeyCode::Enter
    }

    #[inline]
    pub fn set_path(&mut self, path: &str) {
        self.comp.text_area.set_content(path);
    }
}

impl LineComponent for FileOpener {
    const PROMPT: &'static str = "Path: ";
    const BUTTON: &'static str = "[Enter]";
    const POSITION: isize = 1;
    const EDITABLE: bool = true;

    #[inline]
    fn open(&mut self) -> io::Result<()> {
        self.comp.open()
    }

    fn key_resolve(&mut self, key: KeyEvent) -> io::Result<()> {
        if key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT {
            if TextArea::is_editing_key(key.code) {
                self.comp.edit(key.code)?;
            }
        }
        return Ok(());
    }
}
