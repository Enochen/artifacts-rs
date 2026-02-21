use crate::models;
use serde::{Deserialize, Serialize};

/// AchievementRewardsSchema : Schema for achievement rewards including gold and items.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AchievementRewardsSchema {
    /// Gold rewards.
    #[serde(rename = "gold", skip_serializing_if = "Option::is_none")]
    pub gold: Option<i32>,
    /// Item rewards.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::RewardItemSchema>>,
}

impl AchievementRewardsSchema {
    /// Schema for achievement rewards including gold and items.
    pub fn new() -> AchievementRewardsSchema {
        AchievementRewardsSchema {
            gold: None,
            items: None,
        }
    }
}
