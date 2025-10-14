use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BadgeSchema {
    /// Code of the badge. This is the badge's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// Season of the badge.
    #[serde(rename = "season", skip_serializing_if = "Option::is_none")]
    pub season: Option<i32>,
    /// Description of the badge.
    #[serde(rename = "description")]
    pub description: String,
    /// Conditions to get the badge.
    #[serde(rename = "conditions")]
    pub conditions: Vec<models::BadgeConditionSchema>,
}

impl BadgeSchema {
    pub fn new(
        code: String,
        description: String,
        conditions: Vec<models::BadgeConditionSchema>,
    ) -> BadgeSchema {
        BadgeSchema {
            code,
            season: None,
            description,
            conditions,
        }
    }
}
