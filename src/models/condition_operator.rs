use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum ConditionOperator {
    #[serde(rename = "eq")]
    #[default]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "cost")]
    Cost,
    #[serde(rename = "has_item")]
    HasItem,
    #[serde(rename = "achievement_unlocked")]
    AchievementUnlocked,
}

impl std::fmt::Display for ConditionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Ne => write!(f, "ne"),
            Self::Gt => write!(f, "gt"),
            Self::Lt => write!(f, "lt"),
            Self::Cost => write!(f, "cost"),
            Self::HasItem => write!(f, "has_item"),
            Self::AchievementUnlocked => write!(f, "achievement_unlocked"),
        }
    }
}
