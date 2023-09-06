use std::io;

use crossterm::{style::Stylize, event::KeyCode};

use crate::{editor::{text_area::TextArea, direction::Direction}, utils::{Cursor, Terminal}};

pub struct ComponentController {
    pub prompt: &'static str,
    pub button: &'static str,
    pub text_area: TextArea,

    pub position: usize, // position to show
    pub editable: bool,
}

impl ComponentController {
    pub fn open(&mut self) -> io::Result<()> {
        Cursor::move_to_row(self.position)?;
        Cursor::move_to_col(0)?;
        print!("{}", self.prompt.bold().black().on_white());

        Cursor::move_to_col(Terminal::width() - self.button.len())?;
        print!("{}", self.button.bold().black().on_white());

        self.text_area.render()?;
        self.text_area.move_cursor_to_end()?;
        return Ok(());
    }

    #[inline]
    pub fn is_edit_key(key: &KeyCode) -> bool {
        match key {
            KeyCode::Backspace
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::Char(_) => true,
            _ => false,
        }
    }

    pub fn edit(&mut self, key: KeyCode) -> io::Result<()> {
        if !self.editable {
            return Ok(());
        }

        let text_area = &mut self.text_area;
        match key {
            KeyCode::Backspace => text_area.delete_char()?,
            KeyCode::Left => text_area.move_cursor_horizontal(Direction::Left)?,
            KeyCode::Right => text_area.move_cursor_horizontal(Direction::Right)?,
            KeyCode::Char(ch) => text_area.insert_char(ch)?,
            _ => unreachable!()
        }
        return Ok(());
    }
}
