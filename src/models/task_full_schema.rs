use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TaskFullSchema {
    /// Task objective.
    #[serde(rename = "code")]
    pub code: String,
    /// Task level.
    #[serde(rename = "level")]
    pub level: i32,
    /// The type of task.
    #[serde(rename = "type")]
    pub r#type: models::TaskType,
    /// Minimum amount of task.
    #[serde(rename = "min_quantity")]
    pub min_quantity: i32,
    /// Maximum amount of task.
    #[serde(rename = "max_quantity")]
    pub max_quantity: i32,
    #[serde(rename = "skill", deserialize_with = "Option::deserialize")]
    pub skill: Option<String>,
    /// Rewards.
    #[serde(rename = "rewards")]
    pub rewards: Box<models::RewardsSchema>,
}

impl TaskFullSchema {
    pub fn new(
        code: String,
        level: i32,
        r#type: models::TaskType,
        min_quantity: i32,
        max_quantity: i32,
        skill: Option<String>,
        rewards: models::RewardsSchema,
    ) -> TaskFullSchema {
        TaskFullSchema {
            code,
            level,
            r#type,
            min_quantity,
            max_quantity,
            skill,
            rewards: Box::new(rewards),
        }
    }
}
