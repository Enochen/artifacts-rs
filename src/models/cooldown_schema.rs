use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CooldownSchema {
    /// The total seconds of the cooldown.
    #[serde(rename = "total_seconds")]
    pub total_seconds: i32,
    /// The remaining seconds of the cooldown.
    #[serde(rename = "remaining_seconds")]
    pub remaining_seconds: i32,
    /// The start of the cooldown.
    #[serde(rename = "started_at")]
    pub started_at: String,
    /// The expiration of the cooldown.
    #[serde(rename = "expiration")]
    pub expiration: String,
    /// The reason of the cooldown.
    #[serde(rename = "reason")]
    pub reason: models::ActionType,
}

impl CooldownSchema {
    pub fn new(
        total_seconds: i32,
        remaining_seconds: i32,
        started_at: String,
        expiration: String,
        reason: models::ActionType,
    ) -> CooldownSchema {
        CooldownSchema {
            total_seconds,
            remaining_seconds,
            started_at,
            expiration,
            reason,
        }
    }
}
