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

impl crate::traits::IntoData for BankExtensionTransactionResponseSchema {
    type Data = Box<models::BankExtensionTransactionSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
