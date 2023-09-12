use std::io;

use crate::{
    editor::{direction::Direction, text_area::TextArea},
    utils::{number_bit_count, Cursor, Terminal},
};
use crossterm::style::Stylize;

pub struct EditorLine {
    text_area: TextArea,
}

// state methods
impl EditorLine {
    #[inline]
    pub fn is_at_line_start(&self) -> io::Result<bool> {
        Ok(self.text_area.state_left()?.is_at_area_start)
    }
    #[inline]
    pub fn is_at_line_end(&self) -> io::Result<bool> {
        Ok(self.text_area.state_right()?.is_at_area_end)
    }
}

// editing methods
impl EditorLine {
    #[inline]
    pub fn move_cursor_to_start(&mut self, label_width: usize) -> io::Result<()> {
        self.update_label_width(label_width);
        self.text_area.move_cursor_to_start()
    }
    #[inline]
    pub fn move_cursor_to_end(&mut self, label_width: usize) -> io::Result<()> {
        self.update_label_width(label_width);
        self.text_area.move_cursor_to_end()
    }
    #[inline]
    pub fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        self.text_area.move_cursor_horizontal(dir)
    }

    #[inline]
    pub fn insert_char(&mut self, ch: char) -> io::Result<()> {
        self.text_area.insert_char(ch)
    }
    #[inline]
    pub fn delete_char(&mut self) -> io::Result<()> {
        self.text_area.delete_char()
    }
}

impl EditorLine {
    pub fn new(width: usize) -> Self {
        Self {
            text_area: TextArea::new(width, 1),
        }
    }

    pub fn render(&mut self, index: usize, label_width: usize) -> io::Result<()> {
        Cursor::move_to_col(0)?;
        Terminal::clear_after_cursor()?;

        // display label
        let index_width = number_bit_count(index);
        let space_width = label_width - index_width;
        let line_label_str =
            format!("{}{}", index.to_string().black(), " ".repeat(space_width)).on_white();
        print!("{}", line_label_str);

        self.text_area.margin_left = label_width;
        self.text_area.render()?;
        return Ok(());
    }

    #[inline]
    fn update_label_width(&mut self, new_width: usize) {
        self.text_area.margin_left = new_width;
    }

    #[inline]
    pub fn cursor_pos(&self) -> io::Result<usize> {
        self.text_area.cursor_pos()
    }

    #[inline]
    pub fn content<'a>(&'a self) -> &'a str {
        self.text_area.content()
    }

    #[inline]
    pub fn push_str(&mut self, str: &str) {
        self.text_area.push_str(str);
    }

    #[inline]
    pub fn truncate(&mut self) -> io::Result<String> {
        self.text_area.truncate()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.text_area.len()
    }
}
