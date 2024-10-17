use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonsterResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::MonsterSchema>,
}

impl MonsterResponseSchema {
    pub fn new(data: models::MonsterSchema) -> MonsterResponseSchema {
        MonsterResponseSchema {
            data: Box::new(data),
        }
    }
}
