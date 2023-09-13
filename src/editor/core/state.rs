use std::fmt;

use super::EditorMode;

#[derive(Clone, Copy)]
pub enum EditorState {
    Saved,
    Modified,

    // states for components
    Saving,
    Finding,
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

impl From<EditorMode> for EditorState {
    fn from(value: EditorMode) -> Self {
        match value {
            EditorMode::Saving => Self::Saving,
            EditorMode::Positioning => Self::Positioning,
            EditorMode::Finding => Self::Finding,
            _ => unreachable!()
        }
    }
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Saved => "Saved",
            Self::Modified => "Modified",

            Self::Saving => "Saving",
            Self::Finding => "Finding",
            Self::Positioning => "Positioning",
        };
        write!(f, "{}", str)
    }
}
