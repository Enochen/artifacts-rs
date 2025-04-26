use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankGoldTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BankGoldTransactionSchema>,
}

impl BankGoldTransactionResponseSchema {
    pub fn new(data: models::BankGoldTransactionSchema) -> BankGoldTransactionResponseSchema {
        BankGoldTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for BankGoldTransactionResponseSchema {
    type Data = Box<models::BankGoldTransactionSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
