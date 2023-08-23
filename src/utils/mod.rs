mod number_bit_count;
mod logger;

pub mod terminal;
pub mod cursor;

use std::fmt::Display;

pub use number_bit_count::number_bit_count;
pub use logger::log;

pub use terminal::Terminal;
pub use cursor::Cursor;

// this function is used to replace Rust macro `println!`
// since the println! macro can not normally
// make new line in raw_mode.
pub fn print_line<T: Display>(content: T) {
    print!("{}\r\n", content);
    Terminal::flush().expect("IO Error");
}
