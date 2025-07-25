use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ItemType {
    #[serde(rename = "utility")]
    Utility,
    #[serde(rename = "body_armor")]
    BodyArmor,
    #[serde(rename = "weapon")]
    Weapon,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "leg_armor")]
    LegArmor,
    #[serde(rename = "helmet")]
    Helmet,
    #[serde(rename = "boots")]
    Boots,
    #[serde(rename = "shield")]
    Shield,
    #[serde(rename = "amulet")]
    Amulet,
    #[serde(rename = "ring")]
    Ring,
    #[serde(rename = "artifact")]
    Artifact,
    #[serde(rename = "currency")]
    Currency,
    #[serde(rename = "consumable")]
    Consumable,
    #[serde(rename = "rune")]
    Rune,
    #[serde(rename = "bag")]
    Bag,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Utility => write!(f, "utility"),
            Self::BodyArmor => write!(f, "body_armor"),
            Self::Weapon => write!(f, "weapon"),
            Self::Resource => write!(f, "resource"),
            Self::LegArmor => write!(f, "leg_armor"),
            Self::Helmet => write!(f, "helmet"),
            Self::Boots => write!(f, "boots"),
            Self::Shield => write!(f, "shield"),
            Self::Amulet => write!(f, "amulet"),
            Self::Ring => write!(f, "ring"),
            Self::Artifact => write!(f, "artifact"),
            Self::Currency => write!(f, "currency"),
            Self::Consumable => write!(f, "consumable"),
            Self::Rune => write!(f, "rune"),
            Self::Bag => write!(f, "bag"),
        }
    }
}

impl Default for ItemType {
    fn default() -> ItemType {
        Self::Utility
    }
}
