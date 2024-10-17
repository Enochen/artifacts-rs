use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeTransactionListSchema>,
}

impl GeTransactionResponseSchema {
    pub fn new(data: models::GeTransactionListSchema) -> GeTransactionResponseSchema {
        GeTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}
