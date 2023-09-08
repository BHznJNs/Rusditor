use std::{fmt, io};

use crate::utils::Cursor;

// use to indicate virtual cursor position
// in editing area.
pub struct CursorPos {
    pub row: usize,
    pub col: usize,
}

impl CursorPos {
    #[inline]
    pub fn save_pos(&mut self) -> io::Result<()> {
        (self.row, self.col) = (Cursor::pos_row()?, Cursor::pos_col()?);
        return Ok(());
    }

    #[inline]
    pub fn restore_pos(&self) -> io::Result<()> {
        Cursor::move_to_row(self.row)?;
        Cursor::move_to_col(self.col)?;
        return Ok(());
    }
}

impl fmt::Display for CursorPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ln {}, Col {}", self.row, self.col)
    }
}
