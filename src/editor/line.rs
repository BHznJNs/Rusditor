use std::io;

use crate::utils::{log, number_bit_count, Cursor, Terminal};
use crossterm::style::Stylize;

use super::direction::Direction;

#[derive(Debug)]
pub struct Line {
    pub content: String,

    overflow_left: usize,
    overflow_right: usize,

    index_cached: usize,
    label_width_cached: usize,
}

pub struct LineStateLeft {
    pub is_at_left_end: bool,
    pub is_at_line_start: bool,
}
pub struct LineStateRight {
    pub is_at_right_end: bool,
    pub is_at_line_end: bool,
}

// state calculating methods
impl Line {
    #[inline]
    fn is_at_left_end(&self) -> io::Result<bool> {
        return Ok(Cursor::pos_col()? == self.label_width_cached);
    }
    #[inline]
    fn is_at_right_end(&self) -> io::Result<bool> {
        return Ok(Cursor::pos_col()? == Terminal::width() - 1);
    }

    pub fn state_left(&self) -> io::Result<LineStateLeft> {
        let is_at_left_end = self.is_at_left_end()?;
        let is_at_line_start = is_at_left_end && self.overflow_left == 0;
        return Ok(LineStateLeft {
            is_at_left_end,
            is_at_line_start,
        });
    }
    pub fn state_right(&self) -> io::Result<LineStateRight> {
        let cursor_pos_col = Cursor::pos_col()?;
        let is_at_right_end = self.is_at_right_end()?;
        let is_at_line_end = cursor_pos_col == (self.len() + self.label_width_cached)
            || cursor_pos_col == (self.len() - self.overflow_left + self.label_width_cached);
        return Ok(LineStateRight {
            is_at_right_end,
            is_at_line_end,
        });
    }
}

// editing methods
impl Line {
    fn visible_area_width(&self) -> usize {
        let term_width = Terminal::width();
        return term_width - self.label_width_cached - 1;
    }
    fn overflow_refresh(&mut self) {
        let visible_area_width = self.visible_area_width();
        if self.len() > visible_area_width {
            self.overflow_left = self.len() - visible_area_width - self.overflow_right;
        } else {
            self.overflow_left = 0;
            self.overflow_right = 0;
        }
    }

    pub fn move_cursor_to_start(&mut self, label_width: usize) -> io::Result<()> {
        if self.len() >= self.visible_area_width() {
            self.overflow_right += self.overflow_left;
            self.overflow_left = 0;
            self.render(self.index_cached, label_width)?;
        }
        Cursor::move_to_col(label_width)?;
        return Ok(());
    }
    pub fn move_cursor_to_end(&mut self, label_width: usize) -> io::Result<()> {
        if self.len() >= self.visible_area_width() {
            Cursor::move_to_col(Terminal::width() - 1)?;
            self.overflow_left += self.overflow_right;
            self.overflow_right = 0;
            self.render(self.index_cached, label_width)?;
        } else {
            let line_end_pos = label_width + self.len();
            Cursor::move_to_col(line_end_pos)?;
        }
        return Ok(());
    }

    pub fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        match dir {
            Direction::Left => {
                if self.is_at_left_end()? {
                    self.overflow_left -= 1;
                    self.overflow_right += 1;
                } else {
                    Cursor::left(1)?;
                    return Ok(()); // skip rerender
                }
            }
            Direction::Right => {
                if self.is_at_right_end()? {
                    self.overflow_right -= 1;
                    self.overflow_left += 1;
                } else {
                    Cursor::right(1)?;
                    return Ok(()); // skip rerender
                }
            }
            _ => unreachable!(),
        }
        Cursor::save_pos()?;
        self.render(self.index_cached, self.label_width_cached)?;
        Cursor::restore_pos()?;
        return Ok(());
    }

    pub fn insert_char(&mut self, ch: char) -> io::Result<()> {
        let cursor_pos = Cursor::pos_col()?;
        let LineStateRight { is_at_line_end, .. } = self.state_right()?;

        if is_at_line_end {
            self.content.push(ch);
        } else {
            let insert_pos = cursor_pos - self.label_width_cached + self.overflow_left;
            self.content.insert(insert_pos, ch);
        }

        if self.content.len() > self.visible_area_width() {
            self.overflow_left += 1;
        } else {
            Cursor::right(1)?;
        }

        Cursor::save_pos()?;
        self.render(self.index_cached, self.label_width_cached)?;
        Cursor::restore_pos()?;
        return Ok(());
    }

    pub fn delete_char(&mut self) -> io::Result<()> {
        let LineStateRight { is_at_line_end, .. } = self.state_right()?;

        if is_at_line_end {
            self.content.pop();
        } else {
            let remove_pos = Cursor::pos_col()? - self.label_width_cached + self.overflow_left - 1;
            self.content.remove(remove_pos);
        }

        if self.content.len() >= self.visible_area_width() {
            self.overflow_left -= 1;
        } else {
            Cursor::left(1)?;
        }

        Cursor::save_pos()?;
        Terminal::clear_after_cursor()?;
        self.render(self.index_cached, self.label_width_cached)?;
        Cursor::restore_pos()?;
        return Ok(());
    }
}

impl Line {
    pub fn new(index: usize, label_width: usize) -> Self {
        Self {
            content: String::new(),

            overflow_left: 0,
            overflow_right: 0,

            index_cached: index,
            label_width_cached: label_width,
        }
    }

    pub fn render(&mut self, index: usize, label_width: usize) -> io::Result<()> {
        self.index_cached = index;
        self.label_width_cached = label_width;

        Cursor::hide()?;
        Cursor::move_to_col(0)?;
        Terminal::clear_after_cursor()?;

        // display label
        let index_width = number_bit_count(index);
        let space_width = label_width - index_width;
        let line_label_str =
            format!("{}{}", index.to_string().black(), " ".repeat(space_width)).on_white();
        print!("{}", line_label_str);

        // display content
        let visible_area = Terminal::width() - label_width - 1;
        let rendered_content = if self.len() > visible_area {
            &self.content[self.overflow_left..(self.len() - self.overflow_right)]
        } else {
            &self.content
        };
        print!("{}", rendered_content);

        Cursor::show()?;
        Terminal::flush()?;
        return Ok(());
    }

    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str);
        self.overflow_refresh();
    }

    pub fn truncate(&mut self) -> io::Result<String> {
        let truncate_pos = Cursor::pos_col()? - self.label_width_cached + self.overflow_left;
        let mut res_str = String::new();

        self.content[truncate_pos..].clone_into(&mut res_str);
        self.content.truncate(truncate_pos);
        self.overflow_refresh();
        return Ok(res_str);
    }

    #[inline]
    pub fn len(&self) -> usize {
        return self.content.len();
    }
}
