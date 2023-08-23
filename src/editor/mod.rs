mod direction;
mod init;
mod line;

use std::io;

use crossterm::{
    event::{KeyCode, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::utils::{log, number_bit_count, Cursor, Terminal};

use direction::Direction;
use init::EditorInit;
use line::Line;

pub struct Editor {
    lines: Vec<Line>,
    line_index: usize, // current editing line index

    overflow_top: usize,
    overflow_bottom: usize,
}

// short methods
impl Editor {
    // returns the longest line label width at left-side
    fn label_width(&self) -> usize {
        return number_bit_count(self.lines.len()) + 1;
    }

    fn visible_area_height(&self) -> usize {
        let term_height = Terminal::height();
        // `2` here means the top and bottom border.
        return term_height - 2;
    }
}

// editing methods
impl Editor {
    fn render_all(&mut self) -> io::Result<()> {
        Cursor::move_to_row(1)?;
        Cursor::move_to_col(0)?;

        let label_width = self.label_width();
        let line_count = self.lines.len();
        let lines_to_render =
            &mut self.lines[self.overflow_top..(line_count - self.overflow_bottom)];
        let lines_to_render_count = lines_to_render.len();
        let mut line_index = self.overflow_top + 1;

        for line in lines_to_render {
            line.render(line_index, label_width)?;

            Cursor::down(1)?;
            line_index += 1;
        }

        // initialize the unused lines
        let visible_area_height = self.visible_area_height();
        if lines_to_render_count < visible_area_height {
            let diff = visible_area_height - lines_to_render_count;
            for _ in 0..diff {
                Cursor::move_to_col(0)?;
                Terminal::clear_after_cursor()?;
                print!("{}", " ".repeat(label_width).on_white());
                Cursor::down(1)?;
            }
        }
        return Ok(());
    }

    fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        let label_width = self.label_width();
        let current_line = &mut self.lines[self.line_index - 1];

        match dir {
            Direction::Left => {
                let line_state_left = current_line.left_state()?;
                if line_state_left.is_at_line_start {
                    if self.line_index > 1 {
                        self.move_cursor_vertical(Direction::Up)?;
                        let current_line = self.lines.get_mut(self.line_index - 1).unwrap();
                        current_line.move_cursor_to_end(label_width)?;
                    }
                    return Ok(());
                }
            }
            Direction::Right => {
                let line_state_right = current_line.right_state()?;
                if line_state_right.is_at_line_end {
                    if self.line_index < self.lines.len() {
                        self.move_cursor_vertical(Direction::Down)?;
                        let current_line = self.lines.get_mut(self.line_index - 1).unwrap();
                        current_line.move_cursor_to_start(label_width)?;
                    }
                    return Ok(());
                }
            }
            _ => unreachable!()
        }
        current_line.move_cursor_horizontal(dir)?;
        return Ok(());
    }
    fn move_cursor_vertical(&mut self, dir: Direction) -> io::Result<()> {
        let target_line = match dir {
            Direction::Up => {
                // let option_previous_line = self.lines.get(self.line_index - 2);
                if self.line_index <= 1 {
                    return Ok(());
                }
                let previous_line = self.lines.get(self.line_index - 2).unwrap();
                self.line_index -= 1;
                Cursor::up(1)?;
                previous_line
            }
            Direction::Down => {
                let Some(next_line) = self.lines.get(self.line_index) else {
                    return Ok(());
                };
                self.line_index += 1;
                Cursor::down(1)?;
                next_line
            }
            _ => unreachable!(),
        };

        let label_width = self.label_width();
        let cursor_pos = Cursor::pos_col()?;
        // if target_line is shorter than current line
        if cursor_pos - label_width > target_line.len() {
            Cursor::left(cursor_pos - label_width - target_line.len())?;
        }
        return Ok(());
    }

    fn insert_new_line(&mut self) -> io::Result<()> {
        let cursor_pos = Cursor::pos_row()?;

        // vertical-end: `1` here means the top border,
        // subtracting 1 since the cursor position is start from 0.
        let is_at_text_end = cursor_pos == (self.lines.len() + 1 - 1)
            || cursor_pos == (self.lines.len() - self.overflow_top + 1 - 1);

        self.line_index += 1;
        let new_line = Line::new(self.line_index);

        if is_at_text_end {
            self.lines.push(new_line);
        } else {
            let insert_pos = cursor_pos + self.overflow_top;
            self.lines.insert(insert_pos, new_line);
        }

        if self.lines.len() > self.visible_area_height() {
            self.overflow_top += 1;
        } else {
            Cursor::down(1)?;
        }

        Cursor::save_pos()?;
        self.render_all()?;
        Cursor::restore_pos()?;

        let label_width = self.label_width();
        Cursor::move_to_col(label_width)?;
        return Ok(());
    }

    fn insert_char(&mut self, ch: char) -> io::Result<()> {
        let current_line = &mut self.lines[self.line_index - 1];
        current_line.insert_char(ch)?;
        return Ok(());
    }

    fn delete_line(&mut self) -> io::Result<()> {
        let cursor_pos = Cursor::pos_row()?;

        let mut label_width = self.label_width();
        if self.lines.len() % 10 == 0 {
            label_width -= 1;
        }

        let is_at_text_end = cursor_pos == (self.lines.len() + 1 - 1)
            || cursor_pos == (self.lines.len() - self.overflow_top + 1 - 1);

        let (previous_line, poped_line) = if is_at_text_end {
            let last_line = self.lines.pop().unwrap();
            let previous_line = self.lines.last_mut();
            (previous_line, last_line)
        } else {
            let remove_pos = Cursor::pos_col()? - 1 + self.overflow_top;
            let removed_line = self.lines.remove(remove_pos);
            let previous_line = self.lines.get_mut(remove_pos - 1);
            (previous_line, removed_line)
        };

        self.line_index -= 1;
        if let Some(line) = previous_line {

            line.push_str(&poped_line.content);
            line.move_cursor_to_end(label_width)?;
        }

        if self.lines.len() >= self.visible_area_height() {
            self.overflow_top -= 1;
        } else {
            Cursor::up(1)?;
        }

        // rerender
        Cursor::save_pos()?;
        self.render_all()?;
        Cursor::restore_pos()?;
        return Ok(());
    }

    fn delete(&mut self) -> io::Result<()> {
        let cursor_pos = Cursor::pos_col()?;
        let label_width = self.label_width();
        if cursor_pos == label_width && self.line_index == 1 {
            // when at the start of the first line.
            return Ok(());
        }

        let current_line = &mut self.lines[self.line_index - 1];
        let line_left_state = current_line.left_state()?;
        if line_left_state.is_at_line_start {
            self.delete_line()?;
        } else {
            current_line.delete_char()?;
        }
        return Ok(());
    }
}

// public methods
impl Editor {
    pub fn new() -> Self {
        Self {
            lines: vec![Line::new(1)],
            line_index: 1,

            overflow_top: 0,
            overflow_bottom: 0,
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Cursor::move_to_left_top()?;

        let term_width = Terminal::width();
        EditorInit::display_title(term_width);
        EditorInit::display_border(term_width)?;

        let label_width = self.label_width();
        self.lines
            .last_mut()
            .unwrap()
            .render(self.line_index, label_width)?;
        return Ok(());
    }

    pub fn cycle(&mut self) -> io::Result<()> {
        loop {
            let Some(key) = Terminal::get_key() else {
                continue;
            };

            if key.modifiers == KeyModifiers::CONTROL {
                let KeyCode::Char(ch) = key.code else {
                    continue;
                };

                match ch {
                    'x' => todo!(),
                    'c' => todo!(),
                    'v' => todo!(),
                    _ => {}
                }
            }

            match key.code {
                // input `Escape` to exit
                KeyCode::Esc => break,

                KeyCode::Up | KeyCode::Down => {
                    self.move_cursor_vertical(Direction::from(key.code))?;
                }
                KeyCode::Left | KeyCode::Right => {
                    self.move_cursor_horizontal(Direction::from(key.code))?;
                }

                KeyCode::Backspace => self.delete()?,
                KeyCode::Enter => self.insert_new_line()?,
                KeyCode::Char(ch) => {
                    if !ch.is_ascii() {
                        // avoid Non-ASCII characters
                        continue;
                    }
                    self.insert_char(ch)?;
                }
                _ => {}
            }
        }

        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        return Ok(());
    }
}
