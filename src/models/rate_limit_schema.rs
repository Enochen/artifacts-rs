use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateLimitSchema {
    /// Type of rate limit.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Value of the rate limit.
    #[serde(rename = "value")]
    pub value: String,
}

impl RateLimitSchema {
    pub fn new(r#type: String, value: String) -> RateLimitSchema {
        RateLimitSchema { r#type, value }
    }
}
