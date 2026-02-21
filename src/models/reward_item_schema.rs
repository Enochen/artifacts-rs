use crate::models;
use serde::{Deserialize, Serialize};

/// RewardItemSchema : Schema for a single item reward.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RewardItemSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl RewardItemSchema {
    /// Schema for a single item reward.
    pub fn new(code: String, quantity: i32) -> RewardItemSchema {
        RewardItemSchema { code, quantity }
    }
}
