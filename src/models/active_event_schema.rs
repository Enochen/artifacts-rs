use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveEventSchema {
    /// Name of the event.
    #[serde(rename = "name")]
    pub name: String,
    /// Map of the event.
    #[serde(rename = "map")]
    pub map: Box<models::MapSchema>,
    /// Previous map skin.
    #[serde(rename = "previous_skin")]
    pub previous_skin: String,
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
        map: models::MapSchema,
        previous_skin: String,
        duration: i32,
        expiration: String,
        created_at: String,
    ) -> ActiveEventSchema {
        ActiveEventSchema {
            name,
            map: Box::new(map),
            previous_skin,
            duration,
            expiration,
            created_at,
        }
    }
}
