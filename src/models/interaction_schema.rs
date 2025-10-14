use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct InteractionSchema {
    /// Content of the map.
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<models::MapContentSchema>>,
    /// Transition to another map.
    #[serde(rename = "transition", skip_serializing_if = "Option::is_none")]
    pub transition: Option<Box<models::TransitionSchema>>,
}

impl InteractionSchema {
    pub fn new() -> InteractionSchema {
        InteractionSchema {
            content: None,
            transition: None,
        }
    }
}
