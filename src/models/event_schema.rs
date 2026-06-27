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
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<models::EventContentSchema>>,
    /// Map list of the event.
    #[serde(rename = "maps")]
    pub maps: Vec<models::EventMapSchema>,
    /// Duration in minutes.
    #[serde(rename = "duration")]
    pub duration: i32,
    /// Rate spawn of the event. (1/rate every minute)
    #[serde(rename = "rate")]
    pub rate: i32,
    /// Cooldown in minutes before the event can be spawned with gems.
    #[serde(rename = "cooldown", skip_serializing_if = "Option::is_none")]
    pub cooldown: Option<i32>,
    /// Price in gems to spawn the event. Null if not purchasable.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    /// Transition to add to the map when event is active.
    #[serde(rename = "transition", skip_serializing_if = "Option::is_none")]
    pub transition: Option<Box<models::TransitionSchema>>,
    /// Gems spawn cooldown expiration datetime (null if not on cooldown).
    #[serde(
        rename = "cooldown_expiration",
        skip_serializing_if = "Option::is_none"
    )]
    pub cooldown_expiration: Option<String>,
}

impl EventSchema {
    pub fn new(
        name: String,
        code: String,
        maps: Vec<models::EventMapSchema>,
        duration: i32,
        rate: i32,
    ) -> EventSchema {
        EventSchema {
            name,
            code,
            content: None,
            maps,
            duration,
            rate,
            cooldown: None,
            price: None,
            transition: None,
            cooldown_expiration: None,
        }
    }
}
