use std::fmt;

#[derive(Clone, Copy)]
pub enum EditorState {
    Saved,
    Modified,

    // states for components
    Saving,
    Positioning,
}

impl EditorState {
    pub fn is_component_state(&self) -> bool {
        match self {
            Self::Saved | Self::Modified => false,
            _ => true,
        }
    }
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Saved => "Saved",
            Self::Modified => "Modified",

            Self::Saving => "Saving",
            Self::Positioning => "Positioning",
        };
        write!(f, "{}", str)
    }
}
