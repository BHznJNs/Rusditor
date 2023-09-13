mod logger;
mod number_bit_count;
mod loop_traverser;

pub mod cursor;
pub mod terminal;

pub use logger::log;
pub use number_bit_count::number_bit_count;

pub use cursor::Cursor;
pub use terminal::Terminal;
pub use loop_traverser::LoopTraverser;

// // this function is used to replace Rust macro `println!`
// // since the println! macro can not normally
// // make new line in raw_mode.
// pub fn print_line<T: Display>(content: T) {
//     print!("{}\r\n", content);
//     Terminal::flush().expect("IO Error");
// }
