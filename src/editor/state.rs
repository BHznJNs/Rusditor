#[derive(Debug)]
pub struct EditorState {
    pub top_end: bool,
    pub bottom_end: bool,
    pub left_end: bool,
    pub right_end: bool,

    pub line_start: bool,
    pub line_end: bool,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            top_end: true,
            bottom_end: false,
            left_end: true,
            right_end: false,
            line_start: true,
            line_end: true,
        }
    }
}
