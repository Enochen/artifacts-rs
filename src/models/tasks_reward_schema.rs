use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksRewardSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl TasksRewardSchema {
    pub fn new(code: String, quantity: i32) -> TasksRewardSchema {
        TasksRewardSchema { code, quantity }
    }
}
