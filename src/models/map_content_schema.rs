use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MapContentSchema {
    /// Type of the content.
    #[serde(rename = "type")]
    pub r#type: models::MapContentType,
    /// Code of the content.
    #[serde(rename = "code")]
    pub code: String,
}

impl MapContentSchema {
    pub fn new(r#type: models::MapContentType, code: String) -> MapContentSchema {
        MapContentSchema { r#type, code }
    }
}
