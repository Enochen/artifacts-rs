use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RateLimitWindowSchema {
    /// Maximum requests allowed in this window.
    #[serde(rename = "limit")]
    pub limit: i32,
    /// Remaining requests in the current window.
    #[serde(rename = "remaining")]
    pub remaining: i32,
    /// UTC datetime when the window resets.
    #[serde(rename = "reset")]
    pub reset: String,
}

impl RateLimitWindowSchema {
    pub fn new(limit: i32, remaining: i32, reset: String) -> RateLimitWindowSchema {
        RateLimitWindowSchema {
            limit,
            remaining,
            reset,
        }
    }
}
