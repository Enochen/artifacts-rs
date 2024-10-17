use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksRewardFullSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Minimum quantity of item.
    #[serde(rename = "min_quantity")]
    pub min_quantity: i32,
    /// Maximum quantity of item.
    #[serde(rename = "max_quantity")]
    pub max_quantity: i32,
    /// Item odds.
    #[serde(rename = "odds")]
    pub odds: f64,
}

impl TasksRewardFullSchema {
    pub fn new(
        code: String,
        min_quantity: i32,
        max_quantity: i32,
        odds: f64,
    ) -> TasksRewardFullSchema {
        TasksRewardFullSchema {
            code,
            min_quantity,
            max_quantity,
            odds,
        }
    }
}
