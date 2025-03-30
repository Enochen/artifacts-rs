use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSchema {
    /// Task objective.
    #[serde(rename = "code")]
    pub code: String,
    /// The type of task.
    #[serde(rename = "type")]
    pub r#type: models::TaskType,
    /// The total required to complete the task.
    #[serde(rename = "total")]
    pub total: i32,
    /// Rewards for completing the task.
    #[serde(rename = "rewards")]
    pub rewards: Box<models::RewardsSchema>,
}

impl TaskSchema {
    pub fn new(
        code: String,
        r#type: models::TaskType,
        total: i32,
        rewards: models::RewardsSchema,
    ) -> TaskSchema {
        TaskSchema {
            code,
            r#type,
            total,
            rewards: Box::new(rewards),
        }
    }
}
