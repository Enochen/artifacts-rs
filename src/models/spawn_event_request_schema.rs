use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SpawnEventRequestSchema {
    /// Code of the event to spawn
    #[serde(rename = "code")]
    pub code: String,
}

impl SpawnEventRequestSchema {
    pub fn new(code: String) -> SpawnEventRequestSchema {
        SpawnEventRequestSchema { code }
    }
}
