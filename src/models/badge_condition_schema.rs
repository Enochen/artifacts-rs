use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BadgeConditionSchema {
    /// Code of the condition.
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "quantity", deserialize_with = "Option::deserialize")]
    pub quantity: Option<i32>,
}

impl BadgeConditionSchema {
    pub fn new(code: String, quantity: Option<i32>) -> BadgeConditionSchema {
        BadgeConditionSchema { code, quantity }
    }
}
