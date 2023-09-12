use std::{
    fs::{self, File},
    io,
    path::Path,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::text_area::TextArea;

use super::core::{Component, ComponentController};

pub struct FileSaver {
    editor_content: String,
    comp: ComponentController,
}

impl FileSaver {
    const DEFAULT_FILE_NAME: &str = "temp.txt";

    pub fn new() -> Self {
        let mut text_area = TextArea::new(Self::PROMPT.len(), Self::BUTTON.len());
        text_area.set_content(Self::DEFAULT_FILE_NAME);

        Self {
            editor_content: String::new(),
            comp: ComponentController {
                prompt: Self::PROMPT,
                button: Self::BUTTON,
                text_area,
                position: 1,
                editable: true,
            },
        }
    }

    fn save(&self) -> io::Result<()> {
        let target_path_str = self.comp.text_area.content();
        let target_path = Path::new(target_path_str);

        if !target_path.exists() {
            File::create(target_path)?;
        }
        let bytes_to_write = self.editor_content.as_bytes();
        fs::write(target_path_str, bytes_to_write)?;
        return Ok(());
    }

    #[inline]
    pub fn is_save_callback_key(key: KeyEvent) -> bool {
        key.modifiers == KeyModifiers::NONE && key.code == KeyCode::Enter
    }

    #[inline]
    pub fn set_content(&mut self, content: String) {
        self.editor_content = content;
    }

    #[inline]
    pub fn set_path(&mut self, path: &str) {
        self.comp.text_area.set_content(path);
    }
}

impl Component for FileSaver {
    const PROMPT: &'static str = "Path: ";
    const BUTTON: &'static str = "[Enter]";

    #[inline]
    fn open(&mut self) -> io::Result<()> {
        self.comp.open()
    }

    fn key_resolve(&mut self, key: KeyCode) -> io::Result<()> {
        match key {
            KeyCode::Enter => self.save()?,
            k if ComponentController::is_editing_key(k) => self.comp.edit(key)?,
            _ => {}
        }
        return Ok(());
    }
}
