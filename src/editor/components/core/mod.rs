mod controller;

pub(super) use controller::ComponentController;

use std::io;

use crossterm::event::KeyCode;

use crate::editor::text_area::TextArea;

pub trait Component {
    const PROMPT: &'static str;
    const BUTTON: &'static str;
    const POSITION: isize;
    const EDITABLE: bool;

    fn init_controller() -> ComponentController {
        ComponentController {
            prompt: Self::PROMPT,
            button: Self::BUTTON,
            text_area: TextArea::new(Self::PROMPT.len(), Self::BUTTON.len()),

            position: Self::POSITION,
            editable: Self::EDITABLE,
        }
    }
    fn open(&mut self) -> io::Result<()>;
    fn key_resolve(&mut self, key: KeyCode) -> io::Result<()>;
}
