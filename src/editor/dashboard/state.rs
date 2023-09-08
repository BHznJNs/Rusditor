use std::fmt;

pub enum EditorState {
    Saving,
    Saved,

    Modified,
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Saving => "Saving",
            Self::Saved => "Saved",
            Self::Modified => "Modified",
        };
        write!(f, "{}", str)
    }
}
