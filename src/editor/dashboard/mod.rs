mod cursor_pos;
mod state;

use std::io;
use crossterm::style::Stylize;

pub use state::EditorState;
use cursor_pos::CursorPos;
use crate::utils::{Cursor, Terminal};

pub struct EditorDashboard {
    cursor_pos: CursorPos,
    state: EditorState,

    // this cursor position is used to temporarily
    // save and restore cursor. 
    temp_cursor_pos: CursorPos,
}

impl EditorDashboard {
    pub fn new() -> Self {
        Self {
            cursor_pos: CursorPos { row: 1, col: 1 },
            state: EditorState::Saved,

            temp_cursor_pos: CursorPos { row: 1, col: 1 },
        }
    }

    pub fn render(&self) -> io::Result<()> {
        // move cursor to start of the last row
        Cursor::move_to_row(Terminal::height() - 1)?;
        Cursor::move_to_col(0)?;

        let state_str = format!(" {} ", self.state).white().on_dark_red();
        let cursor_pos_str = format!(" {} ", self.cursor_pos).white().on_dark_red();

        // `2` here is space for left-margin and right-margin
        let remain_space =
            Terminal::width() - state_str.content().len() - cursor_pos_str.content().len();
        let divider_str = " ".repeat(remain_space).on_white();

        print!("{state_str}{divider_str}{cursor_pos_str}");
        return Ok(());
    }

    #[inline]
    pub fn set_state(&mut self, state: EditorState) -> io::Result<()> {
        self.state = state;
        self.temp_cursor_pos.save_pos()?;
        self.render()?;
        self.temp_cursor_pos.restore_pos()?;
        return Ok(());
    }

    #[inline]
    pub fn set_cursor_pos(&mut self, row: usize, col: usize) -> io::Result<()> {
        (self.cursor_pos.row, self.cursor_pos.col) = (row, col);
        self.temp_cursor_pos.save_pos()?;
        self.render()?;
        self.temp_cursor_pos.restore_pos()?;
        return Ok(());
    }
}
