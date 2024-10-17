use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::SingleItemSchema>,
}

impl ItemResponseSchema {
    pub fn new(data: models::SingleItemSchema) -> ItemResponseSchema {
        ItemResponseSchema {
            data: Box::new(data),
        }
    }
}
