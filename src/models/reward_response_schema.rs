use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RewardResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::DropRateSchema>,
}

impl RewardResponseSchema {
    pub fn new(data: models::DropRateSchema) -> RewardResponseSchema {
        RewardResponseSchema {
            data: Box::new(data),
        }
    }
}
