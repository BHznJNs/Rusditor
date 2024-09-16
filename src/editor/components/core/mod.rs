mod controller;
mod history;

pub use controller::{LineComponentController, ScreenComponentController};
pub use history::ComponentHistory;

use std::io;

use crossterm::event::KeyEvent;

use crate::editor::text_area::TextArea;

pub trait LineComponent {
    const PROMPT: &'static str;
    const BUTTON: &'static str;
    const POSITION: isize;
    const EDITABLE: bool;

    fn init_controller() -> LineComponentController {
        LineComponentController {
            prompt: Self::PROMPT,
            button: Self::BUTTON,
            text_area: TextArea::new(Self::PROMPT.len(), Self::BUTTON.len()),

            position: Self::POSITION,
            editable: Self::EDITABLE,
        }
    }
    fn open(&mut self) -> io::Result<()>;
    fn key_resolve(&mut self, key: KeyEvent) -> io::Result<()>;
}
