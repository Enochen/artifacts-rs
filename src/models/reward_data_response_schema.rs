use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardDataResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::RewardDataSchema>,
}

impl RewardDataResponseSchema {
    pub fn new(data: models::RewardDataSchema) -> RewardDataResponseSchema {
        RewardDataResponseSchema {
            data: Box::new(data),
        }
    }
}
