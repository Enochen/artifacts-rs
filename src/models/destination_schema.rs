use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DestinationSchema {
    /// The x coordinate of the destination.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y coordinate of the destination.
    #[serde(rename = "y")]
    pub y: i32,
}

impl DestinationSchema {
    pub fn new(x: i32, y: i32) -> DestinationSchema {
        DestinationSchema { x, y }
    }
}
