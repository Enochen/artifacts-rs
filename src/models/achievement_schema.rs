use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
    /// Type of achievement.
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "target", deserialize_with = "Option::deserialize")]
    pub target: Option<String>,
    /// Total to do.
    #[serde(rename = "total")]
    pub total: i32,
    /// Current progress.
    #[serde(rename = "current")]
    pub current: i32,
    #[serde(rename = "completed_at", deserialize_with = "Option::deserialize")]
    pub completed_at: Option<String>,
}

impl AchievementSchema {
    pub fn new(
        name: String,
        code: String,
        description: String,
        points: i32,
        r#type: Type,
        target: Option<String>,
        total: i32,
        current: i32,
        completed_at: Option<String>,
    ) -> AchievementSchema {
        AchievementSchema {
            name,
            code,
            description,
            points,
            r#type,
            target,
            total,
            current,
            completed_at,
        }
    }
}
/// Type of achievement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
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
}

impl Default for Type {
    fn default() -> Type {
        Self::CombatKill
    }
}
