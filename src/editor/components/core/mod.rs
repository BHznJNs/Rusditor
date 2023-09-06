mod controller;

pub(super) use controller::ComponentController;

use std::io;
use crossterm::event::KeyCode;

pub trait Component {
    const PROMPT: &'static str;
    const BUTTON: &'static str;

    fn open(&mut self) -> io::Result<()>;
    fn key_resolve(&mut self, key: KeyCode) -> io::Result<()>;
}
