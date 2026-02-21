use crate::models;
use serde::{Deserialize, Serialize};

/// AccountAchievementObjectiveSchema : Schema for a single objective within an account achievement, including progress.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AccountAchievementObjectiveSchema {
    /// Type of objective.
    #[serde(rename = "type")]
    pub r#type: models::AchievementType,
    /// Target of the objective (e.g., item code, monster code).
    #[serde(rename = "target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Current progress for this objective.
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// Total required for this objective.
    #[serde(rename = "total")]
    pub total: i32,
}

impl AccountAchievementObjectiveSchema {
    /// Schema for a single objective within an account achievement, including progress.
    pub fn new(r#type: models::AchievementType, total: i32) -> AccountAchievementObjectiveSchema {
        AccountAchievementObjectiveSchema {
            r#type,
            target: None,
            progress: None,
            total,
        }
    }
}
