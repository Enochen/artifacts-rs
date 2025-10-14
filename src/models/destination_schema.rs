use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DestinationSchema {
    /// The x coordinate of the destination.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,
    /// The y coordinate of the destination.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,
    /// The map ID of the destination.
    #[serde(rename = "map_id", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
}

impl DestinationSchema {
    pub fn new() -> DestinationSchema {
        DestinationSchema {
            x: None,
            y: None,
            map_id: None,
        }
    }
}
