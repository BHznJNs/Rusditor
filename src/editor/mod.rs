mod components;
mod dashboard;

mod direction;
mod init;
mod line;
mod mode;
mod text_area;

use std::{fs, io};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Stylize,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::utils::{number_bit_count, Cursor, Terminal};

use direction::Direction;
use init::EditorInit;
use line::EditorLine;
use mode::EditorMode;

use components::{Component, EditorComponentManager};
use dashboard::{EditorDashboard, EditorState};

use self::components::FileSaver;

pub struct Editor {
    lines: Vec<EditorLine>,
    line_index: usize, // current editing line index

    overflow_top: usize,
    overflow_bottom: usize,

    mode: EditorMode,
    components: EditorComponentManager,
    dashboard: EditorDashboard,
}

// base value calculating methods
impl Editor {
    #[inline]
    fn label_width(&self) -> usize {
        // returns the longest line label width at left-side
        return number_bit_count(self.lines.len()) + 1;
    }

    #[inline]
    fn label_width_with(&self, value: usize) -> usize {
        // calculate label_width with inputed value
        return number_bit_count(value) + 1;
    }

    #[inline]
    fn visible_area_height(&self) -> usize {
        let term_height = Terminal::height();
        // `2` here means the top and bottom border.
        return term_height - 2;
    }
}

// editing methods
impl Editor {
    fn render_all(&mut self) -> io::Result<()> {
        Cursor::save_pos()?;
        Cursor::move_to_row(1)?;
        Cursor::move_to_col(0)?;

        let label_width = self.label_width();
        let line_count = self.lines.len();

        let line_range = self.overflow_top..(line_count - self.overflow_bottom);
        let lines_to_render = &mut self.lines[line_range.clone()];
        let line_indices = line_range.map(|i| i + 1).collect::<Vec<usize>>();

        let iter = line_indices.into_iter().zip(lines_to_render.iter_mut());
        for (index, line) in iter {
            line.render(index, label_width)?;
            Cursor::down(1)?;
        }

        // initialize the unused lines
        let lines_to_render_count = lines_to_render.len();
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
        Cursor::restore_pos()?;
        return Ok(());
    }

    fn move_cursor_horizontal(&mut self, dir: Direction) -> io::Result<()> {
        let label_width = self.label_width();
        let current_line = &mut self.lines[self.line_index - 1];

        match dir {
            Direction::Left => {
                if current_line.is_at_line_start()? {
                    if self.line_index > 1 {
                        self.move_cursor_vertical(Direction::Up)?;
                        let current_line = self.lines.get_mut(self.line_index - 1).unwrap();
                        current_line.move_cursor_to_end(label_width)?;
                    }
                    return Ok(());
                }
            }
            Direction::Right => {
                if current_line.is_at_line_end()? {
                    if self.line_index < self.lines.len() {
                        self.move_cursor_vertical(Direction::Down)?;
                        let current_line = self.lines.get_mut(self.line_index - 1).unwrap();
                        current_line.move_cursor_to_start(label_width)?;
                    }
                    return Ok(());
                }
            }
            _ => unreachable!(),
        }
        current_line.move_cursor_horizontal(dir)?;
        return Ok(());
    }
    fn move_cursor_vertical(&mut self, dir: Direction) -> io::Result<()> {
        let cursor_pos = Cursor::pos_row()?;
        let target_line = match dir {
            Direction::Up => {
                let is_at_top_side = cursor_pos == 1;
                let is_at_first_line = self.line_index == 1;
                if is_at_first_line {
                    return Ok(());
                }
                self.line_index -= 1;
                if is_at_top_side {
                    self.overflow_top -= 1;
                    self.overflow_bottom += 1;
                    self.render_all()?;
                } else {
                    Cursor::up(1)?;
                }
                self.lines.get(self.line_index - 1).unwrap()
            }
            Direction::Down => {
                let is_at_bottom_side = cursor_pos == Terminal::height() - 2;
                let is_at_last_line = self.line_index == self.lines.len();
                if is_at_last_line {
                    return Ok(());
                }
                self.line_index += 1;
                if is_at_bottom_side {
                    if self.lines.len() == self.visible_area_height() {
                        self.overflow_bottom += 1;
                    } else {
                        self.overflow_top += 1;
                        self.overflow_bottom -= 1;
                    }
                    self.render_all()?;
                } else {
                    Cursor::down(1)?;
                }
                self.lines.get(self.line_index - 1).unwrap()
            }
            _ => unreachable!(),
        };
        let label_width = self.label_width();
        let cursor_pos = Cursor::pos_col()?;

        // if target_line is shorter than current line
        if cursor_pos > target_line.len() + label_width {
            Cursor::left(cursor_pos - label_width - target_line.len())?;
        }
        return Ok(());
    }

    fn insert_line(&mut self) -> io::Result<()> {
        let label_width = self.label_width_with(self.lines.len() + 1);
        let current_line = &mut self.lines[self.line_index - 1];

        let is_at_line_end = current_line.is_at_line_end()?;
        let mut new_line = EditorLine::new(label_width);
        if !is_at_line_end {
            // when input Enter, if cursor is not at line end,
            // truncate current line and push truncated string
            // into the new line.
            let truncated_str = current_line.truncate()?;
            new_line.push_str(&truncated_str);
        }
        new_line.move_cursor_to_start(label_width)?;

        // insert new line
        let insert_pos = Cursor::pos_row()? + self.overflow_top;
        self.lines.insert(insert_pos, new_line);

        self.line_index += 1;
        // scroll
        if self.lines.len() > self.visible_area_height() {
            self.overflow_top += 1;
        } else {
            Cursor::down(1)?;
        }

        self.render_all()?;
        return Ok(());
    }

    fn insert_char(&mut self, ch: char) -> io::Result<()> {
        let current_line = &mut self.lines[self.line_index - 1];
        current_line.insert_char(ch)?;
        return Ok(());
    }

    fn delete_line(&mut self) -> io::Result<()> {
        let label_width = self.label_width_with(self.lines.len() - 1);
        let (previous_line, deleted_line) = {
            let remove_pos = Cursor::pos_row()? + self.overflow_top - 1;
            let removed_line = self.lines.remove(remove_pos);
            let previous_line = self.lines.get_mut(remove_pos - 1);
            (previous_line, removed_line)
        };
        if let Some(line) = previous_line {
            line.push_str(deleted_line.content());
            line.move_cursor_to_end(label_width)?;

            for _ in 0..deleted_line.len() {
                line.move_cursor_horizontal(Direction::Left)?;
            }
        }
        self.line_index -= 1;
        // scroll
        let is_overflowed = self.lines.len() >= self.visible_area_height();
        if is_overflowed && self.overflow_top > 0 {
            self.overflow_top -= 1;
        } else {
            Cursor::up(1)?;
        }
        // rerender
        self.render_all()?;
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
        if current_line.is_at_line_start()? {
            self.delete_line()?;
        } else {
            current_line.delete_char()?;
        }
        return Ok(());
    }
}

// callback resolver methods
impl Editor {
    fn callbacks_resolve(&mut self, key: KeyEvent) -> io::Result<()> {
        match self.mode {
            EditorMode::Saving if FileSaver::is_save_callback_key(key) => {
                self.dashboard.set_state(EditorState::Saved)?;
            }
            _ => {}
        }
        return Ok(());
    }
}

// Non-editing methods
impl Editor {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            line_index: 1,

            overflow_top: 0,
            overflow_bottom: 0,

            mode: EditorMode::Normal,
            components: EditorComponentManager::new(),
            dashboard: EditorDashboard::new(),
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        execute!(io::stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Cursor::move_to_left_top()?;

        EditorInit::display_title();
        EditorInit::display_border()?;
        self.dashboard.render()?;

        // lines.is_empty() == true -> no file reading
        if self.lines.is_empty() {
            // `2` here is the width of line label ("1 ") in terminal.
            self.lines.push(EditorLine::new(2));
        }

        // move cursor to start of first line
        Cursor::move_to_row(1)?;
        let label_width = self.label_width();
        self.lines
            .first_mut()
            .unwrap()
            .move_cursor_to_start(label_width)?;

        self.render_all()?;
        return Ok(());
    }

    pub fn read_file(&mut self, path: &str) -> io::Result<()> {
        self.components.file_saver.set_path(path);
        let file_read_res = fs::read_to_string(path);
        match file_read_res {
            Ok(content) => {
                let file_lines = content.lines();
                let line_count = file_lines.clone().count();
                let visible_area_height = self.visible_area_height();
                let label_width = self.label_width_with(line_count);

                // set `overflow_bottom`
                if line_count > visible_area_height {
                    self.overflow_bottom = line_count - visible_area_height;
                }

                self.lines = file_lines
                    .map(|l| {
                        let mut new_line = EditorLine::new(label_width);
                        new_line.push_str(l);
                        new_line
                    })
                    .collect();
            }
            Err(_) => {
                self.close()?;
                panic!("File reading failed!")
            }
        };
        return Ok(());
    }

    // --- --- --- --- --- ---

    #[inline]
    pub fn close(&self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        return Ok(());
    }

    fn content(&self) -> String {
        let mut buf = String::new();
        let mut iter = self.lines.iter();
        while let Some(line) = iter.next() {
            buf += line.content();
            if iter.len() > 0 {
                buf += "\r\n";
            }
        }
        return buf;
    }

    fn dashboard_cursor_pos_refresh(&mut self) -> io::Result<()> {
        let current_line = &self.lines[self.line_index - 1];
        let current_col = current_line.cursor_pos()? + 1;
        let current_row = Cursor::pos_row()? + self.overflow_top;
        self.dashboard.set_cursor_pos(current_row, current_col)?;
        return Ok(());
    }

    // --- --- --- --- --- ---

    fn toggle_mode(&mut self, mode: EditorMode) -> io::Result<()> {
        match self.mode {
            // set mode
            EditorMode::Normal => {
                Cursor::save_pos()?;
                self.mode = mode;
                match mode {
                    EditorMode::Saving => {
                        let current_content = self.content();
                        self.components.file_saver.set_content(current_content);
                        self.dashboard.set_state(EditorState::Saving)?;
                        &mut self.components.file_saver
                    }
                    _ => unreachable!(),
                }
                .open()?
            }
            // restore to normal mode
            m if m == mode => {
                // restore the covered line
                let label_width = self.label_width();
                let covered_pos = Cursor::pos_row()? + self.overflow_top - 1;
                let covered_line = &mut self.lines[covered_pos];
                covered_line.render(covered_pos + 1, label_width)?;

                Cursor::restore_pos()?;
                self.mode = EditorMode::Normal;
            }
            _ => unreachable!(),
        }
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
                    's' => self.toggle_mode(EditorMode::Saving)?,
                    _ => {}
                }
                continue;
            }

            if self.mode != EditorMode::Normal {
                if key.code == KeyCode::Esc {
                    // use key `Esc` to restore to normal mode
                    self.toggle_mode(self.mode)?;
                    continue;
                }
                self.components.resolve(self.mode, key)?;
                self.callbacks_resolve(key)?;
                continue;
            }

            // will enter matches in normal mode
            match key.code {
                // input `Escape` to exit
                KeyCode::Esc => break,

                KeyCode::Up | KeyCode::Down => {
                    self.move_cursor_vertical(Direction::from(key.code))?;
                }
                KeyCode::Left | KeyCode::Right => {
                    self.move_cursor_horizontal(Direction::from(key.code))?;
                }
                KeyCode::Backspace | KeyCode::Enter | KeyCode::Char(_) => {
                    self.dashboard.set_state(EditorState::Modified)?;
                    match key.code {
                        KeyCode::Backspace => self.delete()?,
                        KeyCode::Enter => self.insert_line()?,
                        KeyCode::Char(ch) => {
                            if !ch.is_ascii() {
                                // avoid Non-ASCII characters
                                continue;
                            }
                            self.insert_char(ch)?;
                        }
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
            self.dashboard_cursor_pos_refresh()?;
        }
        self.close()?;
        return Ok(());
    }
}
