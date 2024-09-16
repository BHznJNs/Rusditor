use std::io;

use crossterm::event::KeyEvent;

use super::core::ScreenComponentController;

pub struct Helper {
    comp: ScreenComponentController,
}

impl Helper {
    pub fn new() -> Self {
        Helper {
            comp: ScreenComponentController {
                content: String::from("Ctrl + s | Open / Close file saving component"),
            }
        }
    }

    #[inline]
    pub fn open(&self) -> io::Result<()> {
        self.comp.render()
    }

    pub fn key_resolve(&self, key: KeyEvent) -> io::Result<()> {
        return Ok(());
    }
}