use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EventContentSchema {
    /// Type of the event.
    #[serde(rename = "type")]
    pub r#type: models::MapContentType,
    /// Code content.
    #[serde(rename = "code")]
    pub code: String,
}

impl EventContentSchema {
    pub fn new(r#type: models::MapContentType, code: String) -> EventContentSchema {
        EventContentSchema { r#type, code }
    }
}
