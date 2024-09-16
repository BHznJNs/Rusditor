use std::io;

use crate::{
    editor::{core::color::EditorColor, direction::Direction, text_area::TextArea},
    utils::{number_bit_count, Cursor, Terminal},
};

pub struct EditorLine {
    text_area: TextArea,
    is_active: bool,

    cached_index: usize,
    cached_label_width: usize,
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
        self.text_area.move_cursor_to_start(true)
    }
    #[inline]
    pub fn move_cursor_to_end(&mut self, label_width: usize) -> io::Result<()> {
        self.update_label_width(label_width);
        self.text_area.move_cursor_to_end(true)
    }
    #[inline]
    pub fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        self.text_area.move_cursor_horizontal(dir, true)
    }

    #[inline]
    pub fn jump_to_word_edge(&mut self, dir: Direction) -> io::Result<()> {
        self.text_area.jump_to_word_edge(dir, true)
    }

    #[inline]
    pub fn insert_char(&mut self, ch: char) -> io::Result<()> {
        self.text_area.insert_char(ch, true)
    }
    #[inline]
    pub fn delete_char(&mut self) -> io::Result<Option<char>> {
        self.text_area.delete_char(true)
    }
}

impl EditorLine {
    pub fn new(width: usize, is_active: bool) -> Self {
        Self {
            text_area: TextArea::new(width, 1),
            is_active,

            cached_index: 0,
            cached_label_width: width,
        }
    }

    pub fn render(&mut self, index: usize, label_width: usize) -> io::Result<()> {
        (self.cached_index, self.cached_label_width) = (index, label_width);
        self.render_label()?;
        self.text_area.margin_left = label_width;
        self.text_area.render()?;

        let saved_col = Cursor::pos_col()?;
        Cursor::move_to_col(Terminal::width() - 1)?;
        print!(" ");
        Cursor::move_to_col(saved_col)?;
        return Ok(());
    }

    pub fn find_all(&self, pat: &str) -> Option<Vec<usize>> {
        let mut text = self.content();
        let mut pos_offset = 0;
        let mut result_vec = vec![];
        while let Some(pos) = text.find(pat) {
            result_vec.push(pos + pos_offset);
            pos_offset += pos + pat.len();
            text = &text[(pos + pat.len())..];
        }

        if result_vec.is_empty() {
            return None;
        } else {
            return Some(result_vec);
        }
    }

    fn render_label(&self) -> io::Result<()> {
        let saved_cursor_pos = Cursor::pos_col()?;
        Cursor::move_to_col(0)?;

        let index_width = number_bit_count(self.cached_index);
        let space_width = self.cached_label_width - index_width;
        let line_label_str = format!("{}{}", self.cached_index, " ".repeat(space_width));
        let line_label_styled = if self.is_active {
            EditorColor::line_active_style(&*line_label_str)
        } else {
            EditorColor::line_disabled_style(&*line_label_str)
        };
        print!("{}", line_label_styled);
        Cursor::move_to_col(saved_cursor_pos)?;
        return Ok(());
    }
    pub fn active(&mut self) -> io::Result<()> {
        self.is_active = true;
        self.render_label()?;
        return Ok(());
    }
    pub fn disable(&mut self) -> io::Result<()> {
        self.is_active = false;
        self.render_label()?;
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
    pub fn content(&self) -> &str {
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

#[test]
fn editorline_find_all_test() {
    let mut line = EditorLine::new(0, false);
    line.push_str("abc  abc  abc");

    assert_eq!(line.find_all("abc"), Some(vec![0, 5, 10]));
}
