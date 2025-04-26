use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankItemTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BankItemTransactionSchema>,
}

impl BankItemTransactionResponseSchema {
    pub fn new(data: models::BankItemTransactionSchema) -> BankItemTransactionResponseSchema {
        BankItemTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for BankItemTransactionResponseSchema {
    type Data = Box<models::BankItemTransactionSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
