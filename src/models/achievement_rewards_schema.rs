use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AchievementRewardsSchema {
    /// Gold rewards.
    #[serde(rename = "gold")]
    pub gold: i32,
}

impl AchievementRewardsSchema {
    pub fn new(gold: i32) -> AchievementRewardsSchema {
        AchievementRewardsSchema { gold }
    }
}
