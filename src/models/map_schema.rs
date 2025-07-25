use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct MapSchema {
    /// Name of the map.
    #[serde(rename = "name")]
    pub name: String,
    /// Skin of the map.
    #[serde(rename = "skin")]
    pub skin: String,
    /// Position X of the map.
    #[serde(rename = "x")]
    pub x: i32,
    /// Position Y of the map.
    #[serde(rename = "y")]
    pub y: i32,
    #[serde(rename = "content", deserialize_with = "Option::deserialize")]
    pub content: Option<Box<models::MapContentSchema>>,
}

impl MapSchema {
    pub fn new(
        name: String,
        skin: String,
        x: i32,
        y: i32,
        content: Option<models::MapContentSchema>,
    ) -> MapSchema {
        MapSchema {
            name,
            skin,
            x,
            y,
            content: content.map(Box::new),
        }
    }
}
