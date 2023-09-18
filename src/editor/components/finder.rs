use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{editor::cursor_pos::EditorCursorPos, utils::LoopTraverser};

use super::{
    core::{ComponentController, ComponentHistory},
    Component,
};

pub struct Finder {
    match_list: LoopTraverser<EditorCursorPos>,

    history: ComponentHistory,
    comp: ComponentController,
}

impl Finder {
    pub fn new() -> Self {
        Self {
            match_list: LoopTraverser::new(true),

            history: ComponentHistory::new(),
            comp: Self::init_controller(),
        }
    }

    #[inline]
    pub fn set_matches(&mut self, pos_list: Vec<EditorCursorPos>) {
        self.match_list.set_content(pos_list);
    }

    #[inline]
    pub fn next<'a>(&'a mut self) -> Option<&'a EditorCursorPos> {
        self.match_list.next()
    }
    #[inline]
    pub fn previous<'a>(&'a mut self) -> Option<&'a EditorCursorPos> {
        self.match_list.previous()
    }

    #[inline]
    pub fn content<'a>(&'a self) -> &'a str {
        self.comp.text_area.content()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.match_list.is_empty()
    }
    #[inline]
    pub fn clear(&mut self) {
        self.match_list.clear()
    }

    // --- --- --- --- --- ---

    #[inline]
    pub fn is_finding_key(key: KeyEvent) -> bool {
        key.modifiers == KeyModifiers::NONE && key.code == KeyCode::Enter
    }
    #[inline]
    pub fn is_reverse_finding_key(key: KeyEvent) -> bool {
        key.modifiers == KeyModifiers::SHIFT && key.code == KeyCode::Enter
    }

}

impl Component for Finder {
    const PROMPT: &'static str = "Find: ";
    const BUTTON: &'static str = "[(Shift) Enter]";
    const POSITION: isize = -1;
    const EDITABLE: bool = true;

    fn open(&mut self) -> io::Result<()> {
        self.comp.open()
    }

    fn key_resolve(&mut self, key: KeyEvent) -> io::Result<()> {
        if !(key.modifiers == KeyModifiers::NONE || key.modifiers == KeyModifiers::SHIFT) {
            return Ok(());
        }

        match key.code {
            KeyCode::Up | KeyCode::Down => {
                let history_content = match key.code {
                    KeyCode::Up => self.history.previous(),
                    KeyCode::Down => self.history.next(),
                    _ => unreachable!(),
                };
                if let Some(str) = history_content {
                    let text_area = &mut self.comp.text_area;
                    text_area.set_content(str);
                    text_area.render()?;
                    text_area.move_cursor_to_end()?;
                }
            }
            KeyCode::Enter => {
                let current_target = self.content();
                self.history.append(current_target.to_owned());
            }
            k if ComponentController::is_editing_key(k) => {
                self.history.reset_index();
                self.comp.edit(k)?;
            }
            _ => {}
        }
        return Ok(());
    }
}
