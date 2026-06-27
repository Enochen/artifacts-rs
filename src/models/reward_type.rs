use crate::models;
use serde::{Deserialize, Serialize};

/// RewardType : Type of season reward.
/// Type of season reward.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum RewardType {
    #[serde(rename = "badge")]
    #[default]
    Badge,
    #[serde(rename = "skin")]
    Skin,
    #[serde(rename = "gold")]
    Gold,
    #[serde(rename = "item")]
    Item,
}

impl std::fmt::Display for RewardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Badge => write!(f, "badge"),
            Self::Skin => write!(f, "skin"),
            Self::Gold => write!(f, "gold"),
            Self::Item => write!(f, "item"),
        }
    }
}
