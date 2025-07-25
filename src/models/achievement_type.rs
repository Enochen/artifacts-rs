use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum AchievementType {
    #[serde(rename = "combat_kill")]
    CombatKill,
    #[serde(rename = "combat_drop")]
    CombatDrop,
    #[serde(rename = "combat_level")]
    CombatLevel,
    #[serde(rename = "gathering")]
    Gathering,
    #[serde(rename = "crafting")]
    Crafting,
    #[serde(rename = "recycling")]
    Recycling,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "use")]
    Use,
}

impl std::fmt::Display for AchievementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CombatKill => write!(f, "combat_kill"),
            Self::CombatDrop => write!(f, "combat_drop"),
            Self::CombatLevel => write!(f, "combat_level"),
            Self::Gathering => write!(f, "gathering"),
            Self::Crafting => write!(f, "crafting"),
            Self::Recycling => write!(f, "recycling"),
            Self::Task => write!(f, "task"),
            Self::Other => write!(f, "other"),
            Self::Use => write!(f, "use"),
        }
    }
}

impl Default for AchievementType {
    fn default() -> AchievementType {
        Self::CombatKill
    }
}
