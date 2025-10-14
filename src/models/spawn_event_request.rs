use crate::models;
use serde::{Deserialize, Serialize};

/// SpawnEventRequest : Model for the request to spawn a specific event
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SpawnEventRequest {
    /// Code of the event to spawn
    #[serde(rename = "code")]
    pub code: String,
}

impl SpawnEventRequest {
    /// Model for the request to spawn a specific event
    pub fn new(code: String) -> SpawnEventRequest {
        SpawnEventRequest { code }
    }
}
