use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeasonBadgeSchema {
    /// Badge code.
    #[serde(rename = "code")]
    pub code: String,
    /// Badge description.
    #[serde(rename = "description")]
    pub description: String,
    /// Required achievement points to earn the badge.
    #[serde(rename = "required_points")]
    pub required_points: i32,
}

impl SeasonBadgeSchema {
    pub fn new(code: String, description: String, required_points: i32) -> SeasonBadgeSchema {
        SeasonBadgeSchema {
            code,
            description,
            required_points,
        }
    }
}
