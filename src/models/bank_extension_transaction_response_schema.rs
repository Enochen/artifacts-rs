use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankExtensionTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BankExtensionTransactionSchema>,
}

impl BankExtensionTransactionResponseSchema {
    pub fn new(
        data: models::BankExtensionTransactionSchema,
    ) -> BankExtensionTransactionResponseSchema {
        BankExtensionTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}
