use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::ResourceSchema>,
}

impl ResourceResponseSchema {
    pub fn new(data: models::ResourceSchema) -> ResourceResponseSchema {
        ResourceResponseSchema {
            data: Box::new(data),
        }
    }
}
