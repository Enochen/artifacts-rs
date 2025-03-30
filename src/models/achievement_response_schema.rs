use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AchievementResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::AchievementSchema>,
}

impl AchievementResponseSchema {
    pub fn new(data: models::AchievementSchema) -> AchievementResponseSchema {
        AchievementResponseSchema {
            data: Box::new(data),
        }
    }
}
