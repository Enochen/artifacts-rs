use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum AccountLeaderboardType {
    #[serde(rename = "achievements_points")]
    #[default]
    AchievementsPoints,
    #[serde(rename = "gold")]
    Gold,
}

impl std::fmt::Display for AccountLeaderboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AchievementsPoints => write!(f, "achievements_points"),
            Self::Gold => write!(f, "gold"),
        }
    }
}
