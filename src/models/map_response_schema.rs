use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::MapSchema>,
}

impl MapResponseSchema {
    pub fn new(data: models::MapSchema) -> MapResponseSchema {
        MapResponseSchema {
            data: Box::new(data),
        }
    }
}
