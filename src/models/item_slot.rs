use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum ItemSlot {
    #[serde(rename = "weapon")]
    Weapon,
    #[serde(rename = "shield")]
    Shield,
    #[serde(rename = "helmet")]
    Helmet,
    #[serde(rename = "body_armor")]
    BodyArmor,
    #[serde(rename = "leg_armor")]
    LegArmor,
    #[serde(rename = "boots")]
    Boots,
    #[serde(rename = "ring1")]
    Ring1,
    #[serde(rename = "ring2")]
    Ring2,
    #[serde(rename = "amulet")]
    Amulet,
    #[serde(rename = "artifact1")]
    Artifact1,
    #[serde(rename = "artifact2")]
    Artifact2,
    #[serde(rename = "artifact3")]
    Artifact3,
    #[serde(rename = "utility1")]
    Utility1,
    #[serde(rename = "utility2")]
    Utility2,
    #[serde(rename = "bag")]
    Bag,
    #[serde(rename = "rune")]
    Rune,
}

impl std::fmt::Display for ItemSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Weapon => write!(f, "weapon"),
            Self::Shield => write!(f, "shield"),
            Self::Helmet => write!(f, "helmet"),
            Self::BodyArmor => write!(f, "body_armor"),
            Self::LegArmor => write!(f, "leg_armor"),
            Self::Boots => write!(f, "boots"),
            Self::Ring1 => write!(f, "ring1"),
            Self::Ring2 => write!(f, "ring2"),
            Self::Amulet => write!(f, "amulet"),
            Self::Artifact1 => write!(f, "artifact1"),
            Self::Artifact2 => write!(f, "artifact2"),
            Self::Artifact3 => write!(f, "artifact3"),
            Self::Utility1 => write!(f, "utility1"),
            Self::Utility2 => write!(f, "utility2"),
            Self::Bag => write!(f, "bag"),
            Self::Rune => write!(f, "rune"),
        }
    }
}

impl Default for ItemSlot {
    fn default() -> ItemSlot {
        Self::Weapon
    }
}
