use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AccountAchievementSchema {
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
    /// Type of achievement.
    #[serde(rename = "type")]
    pub r#type: models::AchievementType,
    #[serde(rename = "target", deserialize_with = "Option::deserialize")]
    pub target: Option<String>,
    /// Total to do.
    #[serde(rename = "total")]
    pub total: i32,
    /// Rewards.
    #[serde(rename = "rewards")]
    pub rewards: Box<models::AchievementRewardsSchema>,
    /// Current progress.
    #[serde(rename = "current")]
    pub current: i32,
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
}

impl AccountAchievementSchema {
    pub fn new(
        name: String,
        code: String,
        description: String,
        points: i32,
        r#type: models::AchievementType,
        target: Option<String>,
        total: i32,
        rewards: models::AchievementRewardsSchema,
        current: i32,
        completed_at: Option<String>,
    ) -> AccountAchievementSchema {
        AccountAchievementSchema {
            name,
            code,
            description,
            points,
            r#type,
            target,
            total,
            rewards: Box::new(rewards),
            current,
            completed_at,
        }
    }
}
