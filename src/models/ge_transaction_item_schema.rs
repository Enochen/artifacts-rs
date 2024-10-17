use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeTransactionItemSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
    /// Item price. Item price validation protects you if the price has changed since you last checked the buy/sale price of an item.
    #[serde(rename = "price")]
    pub price: u32,
}

impl GeTransactionItemSchema {
    pub fn new(code: String, quantity: u32, price: u32) -> GeTransactionItemSchema {
        GeTransactionItemSchema {
            code,
            quantity,
            price,
        }
    }
}
