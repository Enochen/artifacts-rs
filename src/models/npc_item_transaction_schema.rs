use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpcItemTransactionSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// Item price.
    #[serde(rename = "price")]
    pub price: i32,
    /// Total price of the transaction.
    #[serde(rename = "total_price")]
    pub total_price: i32,
}

impl NpcItemTransactionSchema {
    pub fn new(
        code: String,
        quantity: i32,
        price: i32,
        total_price: i32,
    ) -> NpcItemTransactionSchema {
        NpcItemTransactionSchema {
            code,
            quantity,
            price,
            total_price,
        }
    }
}
