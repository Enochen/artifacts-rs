use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ActiveEventSchema {
    /// Name of the event.
    #[serde(rename = "name")]
    pub name: String,
    /// Code of the event.
    #[serde(rename = "code")]
    pub code: String,
    /// Map of the event.
    #[serde(rename = "map")]
    pub map: Box<models::MapSchema>,
    /// Previous map skin.
    #[serde(rename = "previous_map")]
    pub previous_map: Box<models::MapSchema>,
    /// Duration in minutes.
    #[serde(rename = "duration")]
    pub duration: i32,
    /// Expiration datetime.
    #[serde(rename = "expiration")]
    pub expiration: String,
    /// Start datetime.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl ActiveEventSchema {
    pub fn new(
        name: String,
        code: String,
        map: models::MapSchema,
        previous_map: models::MapSchema,
        duration: i32,
        expiration: String,
        created_at: String,
    ) -> ActiveEventSchema {
        ActiveEventSchema {
            name,
            code,
            map: Box::new(map),
            previous_map: Box::new(previous_map),
            duration,
            expiration,
            created_at,
        }
    }
}
