use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BankSchema>,
}

impl BankResponseSchema {
    pub fn new(data: models::BankSchema) -> BankResponseSchema {
        BankResponseSchema {
            data: Box::new(data),
        }
    }
}
