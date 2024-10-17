use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseachievementResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BaseAchievementSchema>,
}

impl BaseachievementResponseSchema {
    pub fn new(data: models::BaseAchievementSchema) -> BaseachievementResponseSchema {
        BaseachievementResponseSchema {
            data: Box::new(data),
        }
    }
}
