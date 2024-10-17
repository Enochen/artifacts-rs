use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskRewardsSchema {
    /// Items rewards.
    #[serde(rename = "items")]
    pub items: Vec<models::SimpleItemSchema>,
}

impl TaskRewardsSchema {
    pub fn new(items: Vec<models::SimpleItemSchema>) -> TaskRewardsSchema {
        TaskRewardsSchema { items }
    }
}
