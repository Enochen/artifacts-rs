use crate::models;
use serde::{Deserialize, Serialize};

/// AchievementObjectiveSchema : Schema for a single objective within an achievement.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AchievementObjectiveSchema {
    /// Type of objective.
    #[serde(rename = "type")]
    pub r#type: models::AchievementType,
    /// Target of the objective (e.g., item code, monster code).
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Total required for this objective.
    #[serde(rename = "total")]
    pub total: i32,
}

impl AchievementObjectiveSchema {
    /// Schema for a single objective within an achievement.
    pub fn new(r#type: models::AchievementType, total: i32) -> AchievementObjectiveSchema {
        AchievementObjectiveSchema {
            r#type,
            target: None,
            total,
        }
    }
}
