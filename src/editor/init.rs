use std::io;

use crossterm::style::Stylize;

use crate::utils::{print_line, Cursor, Terminal};

pub struct EditorInit;

impl EditorInit {
    pub fn display_title(width: usize) {
        let term_width = Terminal::width();
        let title_str = format!("REditor v{}", env!("CARGO_PKG_VERSION"));
        let padding = (width - title_str.len()) / 2;
        let padding_str1 = " ".repeat(padding);
        let padding_str2 = " ".repeat(term_width - title_str.len() - padding);
        print_line(
            format!("{}{}{}", padding_str1, title_str, padding_str2)
                .bold()
                .black()
                .on_white(),
        );
    }

    pub fn display_border(width: usize) -> io::Result<()> {
        // print left and right border
        for _ in 1..Terminal::height() {
            print!("{}", "  ".on_white());
            Cursor::down(1)?;
            Cursor::move_to_col(0)?;
        }
        // print bottom border
        print!("{}", " ".repeat(width).on_white());

        // move cursor to left-top of edit area
        Cursor::move_to_row(1)?;
        Cursor::move_to_col(0)?;

        Terminal::flush()?;
        return Ok(());
    }
}
