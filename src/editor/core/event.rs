use crate::editor::cursor_pos::EditorCursorPos;

#[derive(Clone)]
pub enum EditorOperation {
    InsertChar(char),
    DeleteChar(char),
    InsertLine,
    DeleteLine,
}

impl EditorOperation {
    pub fn rev(&self) -> Self {
        match self {
            Self::InsertChar(ch) => Self::DeleteChar(*ch),
            Self::DeleteChar(ch) => Self::InsertChar(*ch),
            Self::InsertLine => Self::DeleteLine,
            Self::DeleteLine => Self::InsertLine,
        }
    }
}

// --- --- --- --- --- ---

#[derive(Clone)]
pub struct EditorEvent {
    pub op: EditorOperation,

    pub pos_before: EditorCursorPos,
    pub pos_after: EditorCursorPos,
}
