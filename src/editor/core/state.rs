use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum EditorState {
    Saved,
    Modified,

    // states for components
    Saving,
    Opening,
    Positioning,
    Finding,
    Replacing,

    ReadingHelpMsg
}

impl EditorState {
    pub fn is_component_state(&self) -> bool {
        !matches!(self, Self::Saved | Self::Modified)
    }
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Saved => "Saved",
            Self::Modified => "Modified",

            Self::Saving => "Saving",
            Self::Opening => "Opening",
            Self::Positioning => "Positioning",
            Self::Finding => "Finding",
            Self::Replacing => "Replacing",

            Self::ReadingHelpMsg => "Reading",
        };
        write!(f, "{}", str)
    }
}
