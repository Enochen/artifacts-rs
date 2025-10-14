use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TransitionSchema {
    /// ID of the destination map.
    #[serde(rename = "map_id")]
    pub map_id: i32,
    /// Position X of the destination.
    #[serde(rename = "x")]
    pub x: i32,
    /// Position Y of the destination.
    #[serde(rename = "y")]
    pub y: i32,
    /// Layer of the destination.
    #[serde(rename = "layer")]
    pub layer: models::MapLayer,
    /// Conditions for the transition.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<models::ConditionSchema>>,
}

impl TransitionSchema {
    pub fn new(map_id: i32, x: i32, y: i32, layer: models::MapLayer) -> TransitionSchema {
        TransitionSchema {
            map_id,
            x,
            y,
            layer,
            conditions: None,
        }
    }
}
