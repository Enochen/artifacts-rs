use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct OfferingFavorSchema {
    /// Activity receiving the XP favor.
    #[serde(rename = "target_type")]
    pub target_type: TargetType,
    /// Exact monster, resource node, or craft receiving the XP favor.
    #[serde(rename = "target_code")]
    pub target_code: String,
    /// XP multiplier active after accepting the offering.
    #[serde(rename = "xp_multiplier", skip_serializing_if = "Option::is_none")]
    pub xp_multiplier: Option<f64>,
    /// Item drop quantity multiplier. It is 1.5 for monster and resource favors, and 1.0 for craft favors.
    #[serde(rename = "drop_multiplier", skip_serializing_if = "Option::is_none")]
    pub drop_multiplier: Option<f64>,
}

impl OfferingFavorSchema {
    pub fn new(target_type: TargetType, target_code: String) -> OfferingFavorSchema {
        OfferingFavorSchema {
            target_type,
            target_code,
            xp_multiplier: None,
            drop_multiplier: None,
        }
    }
}
/// Activity receiving the XP favor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum TargetType {
    #[serde(rename = "monster")]
    #[default]
    Monster,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "craft")]
    Craft,
}
