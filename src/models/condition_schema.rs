use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ConditionSchema {
    /// Condition code.
    #[serde(rename = "code")]
    pub code: String,
    /// Condition operator.
    #[serde(rename = "operator")]
    pub operator: models::ConditionOperator,
    /// Condition value.
    #[serde(rename = "value")]
    pub value: i32,
}

impl ConditionSchema {
    pub fn new(code: String, operator: models::ConditionOperator, value: i32) -> ConditionSchema {
        ConditionSchema {
            code,
            operator,
            value,
        }
    }
}
