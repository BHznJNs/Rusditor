use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::{
    cursor_pos::EditorCursorPos,
    text_area::TextArea,
};

use super::{core::ComponentController, Component};

pub struct Positioner {
    target: EditorCursorPos,
    comp: ComponentController,
}

impl Positioner {
    const PROMPT: &'static str = "Target: ";
    const BUTTON: &'static str = "[Enter]";

    pub fn new() -> Self {
        let initial_cursor_pos = EditorCursorPos { row: 1, col: 1 };
        let mut text_area = TextArea::new(Self::PROMPT.len(), Self::BUTTON.len());
        text_area.set_content(&initial_cursor_pos.short_display());

        Self {
            target: initial_cursor_pos,
            comp: ComponentController {
                prompt: Self::PROMPT,
                button: Self::BUTTON,
                text_area,
                position: -2,
                editable: true,
            },
        }
    }

    #[inline]
    pub fn set_cursor_pos(&mut self, pos: EditorCursorPos) {
        let pos_str = pos.short_display();
        self.comp.text_area.set_content(&pos_str);
        self.target = pos;
    }

    #[inline]
    pub fn is_positioning_key(key: KeyEvent) -> bool {
        key.modifiers == KeyModifiers::NONE && key.code == KeyCode::Enter
    }

    #[inline]
    pub fn get_target(&self) -> EditorCursorPos {
        self.target.clone()
    }
}

impl Component for Positioner {
    const PROMPT: &'static str = "Target: ";
    const BUTTON: &'static str = "[Enter]";

    #[inline]
    fn open(&mut self) -> io::Result<()> {
        self.comp.open()?;
        self.comp.text_area.move_cursor_to_start()?;
        return Ok(());
    }

    fn key_resolve(&mut self, key: KeyCode) -> io::Result<()> {
        match key {
            KeyCode::Enter => {
                let target_pos_str = self.comp.text_area.content();
                match EditorCursorPos::parse(target_pos_str) {
                    Some(pos) => self.target = pos,
                    None => return Ok(()),
                }
            }
            k if ComponentController::is_editing_key(k) => self.comp.edit(key)?,
            _ => {}
        }
        return Ok(());
    }
}
