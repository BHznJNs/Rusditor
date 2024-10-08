use std::io;

use crossterm::{event::KeyCode, style::Stylize};

use crate::{
    editor::{direction::Direction, text_area::TextArea},
    utils::{Cursor, Terminal},
};

pub struct LineComponentController {
    pub prompt: &'static str,
    pub button: &'static str,
    pub text_area: TextArea,

    // verticle position to show,
    // if less than zero, equal to
    // Terminal::height() - position - 1
    pub position: isize,

    pub editable: bool,
}

impl LineComponentController {
    pub fn open(&mut self) -> io::Result<()> {
        let render_pos = if self.position >= 0 {
            self.position as usize
        } else {
            (Terminal::height() as isize + self.position - 1) as usize
        };

        Cursor::move_to_row(render_pos)?;
        Cursor::move_to_col(0)?;
        print!("{}", self.prompt.bold().black().on_white());

        Cursor::move_to_col(Terminal::width() - self.button.len())?;
        print!("{}", self.button.bold().black().on_white());

        self.text_area.move_cursor_to_end(false)?;
        self.text_area.render()?;
        return Ok(());
    }

    pub fn edit(&mut self, key: KeyCode) -> io::Result<()> {
        if !self.editable {
            return Ok(());
        }

        let text_area = &mut self.text_area;
        match key {
            KeyCode::Backspace => {
                text_area.delete_char(true)?;
            }

            KeyCode::Left => text_area.move_cursor_horizontal(Direction::Left, true)?,
            KeyCode::Right => text_area.move_cursor_horizontal(Direction::Right, true)?,
            KeyCode::Char(ch) => text_area.insert_char(ch, true)?,
            _ => unreachable!(),
        }
        return Ok(());
    }
}

// --- --- --- --- --- ---

pub struct ScreenComponentController {
    pub content: String,
}

impl ScreenComponentController {
    pub fn render(&self) -> io::Result<()> {
        Cursor::move_to_col(1)?;
        Cursor::move_to_row(0)?;

        let term_width = Terminal::width();
        let mut slice = &self.content[..];
        while !slice.is_empty() {
            if slice.len() > term_width {
                print!("{}", &slice[0..term_width]);
                slice = &slice[term_width..];
                Cursor::down(1)?;
            } else {
                print!("{}", slice);
                break;
            }
        }
        return Ok(());
    }
}