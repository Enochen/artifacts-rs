use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpcResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::NpcSchema>,
}

impl NpcResponseSchema {
    pub fn new(data: models::NpcSchema) -> NpcResponseSchema {
        NpcResponseSchema {
            data: Box::new(data),
        }
    }
}
