use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum EffectType {
    #[serde(rename = "equipment")]
    Equipment,
    #[serde(rename = "consumable")]
    Consumable,
    #[serde(rename = "combat")]
    Combat,
}

impl std::fmt::Display for EffectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Equipment => write!(f, "equipment"),
            Self::Consumable => write!(f, "consumable"),
            Self::Combat => write!(f, "combat"),
        }
    }
}

impl Default for EffectType {
    fn default() -> EffectType {
        Self::Equipment
    }
}
