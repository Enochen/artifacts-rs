use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EventMapSchema {
    /// ID of the map.
    #[serde(rename = "map_id")]
    pub map_id: i32,
    /// Position X of the map.
    #[serde(rename = "x")]
    pub x: i32,
    /// Position Y of the map.
    #[serde(rename = "y")]
    pub y: i32,
    /// Layer of the map.
    #[serde(rename = "layer")]
    pub layer: String,
    /// Map skin of the map
    #[serde(rename = "skin")]
    pub skin: String,
}

impl EventMapSchema {
    pub fn new(map_id: i32, x: i32, y: i32, layer: String, skin: String) -> EventMapSchema {
        EventMapSchema {
            map_id,
            x,
            y,
            layer,
            skin,
        }
    }
}
