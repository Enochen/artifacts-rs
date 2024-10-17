use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeItemSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item stock.
    #[serde(rename = "stock")]
    pub stock: i32,
    /// The item's selling price.
    #[serde(rename = "sell_price", skip_serializing_if = "Option::is_none")]
    pub sell_price: Option<i32>,
    /// The item's buying price.
    #[serde(rename = "buy_price", skip_serializing_if = "Option::is_none")]
    pub buy_price: Option<i32>,
    /// The number of items you can buy or sell at the same time.
    #[serde(rename = "max_quantity")]
    pub max_quantity: i32,
}

impl GeItemSchema {
    pub fn new(code: String, stock: i32, max_quantity: i32) -> GeItemSchema {
        GeItemSchema {
            code,
            stock,
            sell_price: None,
            buy_price: None,
            max_quantity,
        }
    }
}
