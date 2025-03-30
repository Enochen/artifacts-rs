use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeCreateOrderTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeOrderTransactionSchema>,
}

impl GeCreateOrderTransactionResponseSchema {
    pub fn new(data: models::GeOrderTransactionSchema) -> GeCreateOrderTransactionResponseSchema {
        GeCreateOrderTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}
