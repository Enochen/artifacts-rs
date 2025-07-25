use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EventSchema {
    /// Name of the event.
    #[serde(rename = "name")]
    pub name: String,
    /// Code of the event.
    #[serde(rename = "code")]
    pub code: String,
    /// Content of the event.
    #[serde(rename = "content")]
    pub content: Box<models::EventContentSchema>,
    /// Map list of the event.
    #[serde(rename = "maps")]
    pub maps: Vec<models::EventMapSchema>,
    /// Duration in minutes.
    #[serde(rename = "duration")]
    pub duration: i32,
    /// Rate spawn of the event. (1/rate every minute)
    #[serde(rename = "rate")]
    pub rate: i32,
}

impl EventSchema {
    pub fn new(
        name: String,
        code: String,
        content: models::EventContentSchema,
        maps: Vec<models::EventMapSchema>,
        duration: i32,
        rate: i32,
    ) -> EventSchema {
        EventSchema {
            name,
            code,
            content: Box::new(content),
            maps,
            duration,
            rate,
        }
    }
}
