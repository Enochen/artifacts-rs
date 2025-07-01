use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeasonSkinSchema {
    /// Skin code.
    #[serde(rename = "code")]
    pub code: String,
    /// Skin description.
    #[serde(rename = "description")]
    pub description: String,
    /// Required achievement points to earn the skin.
    #[serde(rename = "required_points")]
    pub required_points: i32,
}

impl SeasonSkinSchema {
    pub fn new(code: String, description: String, required_points: i32) -> SeasonSkinSchema {
        SeasonSkinSchema {
            code,
            description,
            required_points,
        }
    }
}
