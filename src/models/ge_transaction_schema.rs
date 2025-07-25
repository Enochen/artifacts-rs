use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeTransactionSchema {
    /// Order id.
    #[serde(rename = "id")]
    pub id: String,
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

impl GeTransactionSchema {
    pub fn new(
        id: String,
        code: String,
        quantity: i32,
        price: i32,
        total_price: i32,
    ) -> GeTransactionSchema {
        GeTransactionSchema {
            id,
            code,
            quantity,
            price,
            total_price,
        }
    }
}
