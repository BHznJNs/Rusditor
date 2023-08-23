use crossterm::event::KeyCode;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,

    Left,
    Right,
}

impl From<KeyCode> for Direction {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            _ => unreachable!()
        }
    }
}
