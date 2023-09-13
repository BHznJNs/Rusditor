use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::editor::cursor_pos::EditorCursorPos;

use super::{core::ComponentController, Component};

pub struct Positioner {
    target: EditorCursorPos,
    comp: ComponentController,
}

impl Positioner {
    pub fn new() -> Self {
        let initial_cursor_pos = EditorCursorPos { row: 1, col: 1 };
        let mut controller = Self::init_controller();
        controller
            .text_area
            .set_placeholder(&initial_cursor_pos.short_display());
        return Self {
            target: initial_cursor_pos,
            comp: controller,
        };
    }

    #[inline]
    pub fn set_cursor_pos(&mut self, pos: EditorCursorPos) {
        let pos_str = pos.short_display();
        self.comp.text_area.set_placeholder(&pos_str);
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
    const POSITION: isize = -1;
    const EDITABLE: bool = true;

    #[inline]
    fn open(&mut self) -> io::Result<()> {
        self.comp.open()
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
