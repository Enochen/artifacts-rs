use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapContentSchema {
    /// Type of the content.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Code of the content.
    #[serde(rename = "code")]
    pub code: String,
}

impl MapContentSchema {
    pub fn new(r#type: String, code: String) -> MapContentSchema {
        MapContentSchema { r#type, code }
    }
}
