use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardsSchema {
    /// Items rewards.
    #[serde(rename = "items")]
    pub items: Vec<models::SimpleItemSchema>,
    /// Gold rewards.
    #[serde(rename = "gold")]
    pub gold: i32,
}

impl RewardsSchema {
    pub fn new(items: Vec<models::SimpleItemSchema>, gold: i32) -> RewardsSchema {
        RewardsSchema { items, gold }
    }
}
