use crate::models;
use serde::{Deserialize, Serialize};

/// AchievementSchema : Schema for an achievement definition.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AchievementSchema {
    /// Name of the achievement.
    #[serde(rename = "name")]
    pub name: String,
    /// Code of the achievement.
    #[serde(rename = "code")]
    pub code: String,
    /// Description of the achievement.
    #[serde(rename = "description")]
    pub description: String,
    /// Points of the achievement. Used for the leaderboard.
    #[serde(rename = "points")]
    pub points: i32,
    /// List of objectives that must be completed.
    #[serde(rename = "objectives")]
    pub objectives: Vec<models::AchievementObjectiveSchema>,
    /// Rewards.
    #[serde(rename = "rewards")]
    pub rewards: Box<models::AchievementRewardsSchema>,
}

impl AchievementSchema {
    /// Schema for an achievement definition.
    pub fn new(
        name: String,
        code: String,
        description: String,
        points: i32,
        objectives: Vec<models::AchievementObjectiveSchema>,
        rewards: models::AchievementRewardsSchema,
    ) -> AchievementSchema {
        AchievementSchema {
            name,
            code,
            description,
            points,
            objectives,
            rewards: Box::new(rewards),
        }
    }
}
