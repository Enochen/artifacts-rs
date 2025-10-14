use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BadgeConditionSchema {
    /// Code of the condition.
    #[serde(rename = "code")]
    pub code: String,
    /// Quantity of the condition (if any).
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl BadgeConditionSchema {
    pub fn new(code: String) -> BadgeConditionSchema {
        BadgeConditionSchema {
            code,
            quantity: None,
        }
    }
}
