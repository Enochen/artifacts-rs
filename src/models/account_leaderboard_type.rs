use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountLeaderboardType {
    #[serde(rename = "achievements_points")]
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

impl Default for AccountLeaderboardType {
    fn default() -> AccountLeaderboardType {
        Self::AchievementsPoints
    }
}
