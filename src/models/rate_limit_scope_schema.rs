use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RateLimitScopeSchema {
    #[serde(rename = "second", skip_serializing_if = "Option::is_none")]
    pub second: Option<Box<models::RateLimitWindowSchema>>,
    #[serde(rename = "minute", skip_serializing_if = "Option::is_none")]
    pub minute: Option<Box<models::RateLimitWindowSchema>>,
    #[serde(rename = "hour", skip_serializing_if = "Option::is_none")]
    pub hour: Option<Box<models::RateLimitWindowSchema>>,
    #[serde(rename = "day", skip_serializing_if = "Option::is_none")]
    pub day: Option<Box<models::RateLimitWindowSchema>>,
}

impl RateLimitScopeSchema {
    pub fn new() -> RateLimitScopeSchema {
        RateLimitScopeSchema {
            second: None,
            minute: None,
            hour: None,
            day: None,
        }
    }
}
