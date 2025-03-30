use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventMapSchema {
    /// Position X of the map.
    #[serde(rename = "x")]
    pub x: i32,
    /// Position Y of the map.
    #[serde(rename = "y")]
    pub y: i32,
}

impl EventMapSchema {
    pub fn new(x: i32, y: i32) -> EventMapSchema {
        EventMapSchema { x, y }
    }
}
