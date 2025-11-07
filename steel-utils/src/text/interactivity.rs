use serde::{Deserialize, Serialize};

use super::click::ClickEvent;
use super::hover::HoverEvent;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Interactivity {
    /// Allows for events to occur when the player clicks on text. Only works in chat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    /// Allows for a tooltip to be displayed when the player hovers their mouse over text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}

impl Interactivity {
    pub const fn new() -> Self {
        Self {
            click_event: None,
            hover_event: None,
        }
    }
}

impl Default for Interactivity {
    fn default() -> Self {
        Self::new()
    }
}
