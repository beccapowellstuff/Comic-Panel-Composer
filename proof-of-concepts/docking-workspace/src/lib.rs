//! Minimal shared state for a multi-window desktop workspace experiment.
//! This is not the Comic Panel Composer document model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Panel {
    Canvas,
    Pages,
    Layers,
    Inspector,
}

impl Panel {
    pub const FLOATABLE: [Self; 3] = [Self::Pages, Self::Layers, Self::Inspector];

    pub fn title(self) -> &'static str {
        match self {
            Self::Canvas => "Canvas",
            Self::Pages => "Pages",
            Self::Layers => "Layers",
            Self::Inspector => "Inspector",
        }
    }

    /// The index of an independently detachable tool panel.
    pub fn native_slot(self) -> Option<usize> {
        match self {
            Self::Pages => Some(0),
            Self::Layers => Some(1),
            Self::Inspector => Some(2),
            Self::Canvas => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentState {
    pub selected_page: u8,
    pub layer_visible: bool,
    pub opacity: u8,
}

impl Default for DocumentState {
    fn default() -> Self {
        Self {
            selected_page: 1,
            layer_visible: true,
            opacity: 200,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DocumentState, Panel};

    #[test]
    fn only_tool_panels_are_detachable() {
        assert_eq!(Panel::Canvas.native_slot(), None);
        assert_eq!(Panel::Pages.native_slot(), Some(0));
        assert_eq!(Panel::Layers.native_slot(), Some(1));
        assert_eq!(Panel::Inspector.native_slot(), Some(2));
    }

    #[test]
    fn floatable_panels_have_unique_slots() {
        let slots: Vec<_> = Panel::FLOATABLE
            .into_iter()
            .map(|panel| panel.native_slot().unwrap())
            .collect();
        assert_eq!(slots, [0, 1, 2]);
    }

    #[test]
    fn default_document_state_is_consistent() {
        let state = DocumentState::default();
        assert_eq!(state.selected_page, 1);
        assert!(state.layer_visible);
        assert_eq!(state.opacity, 200);
    }

    #[test]
    fn shared_document_state_round_trips_through_json() {
        let expected = DocumentState {
            selected_page: 3,
            layer_visible: false,
            opacity: 89,
        };
        let json = serde_json::to_string(&expected).unwrap();
        let restored: DocumentState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, expected);
    }
}
