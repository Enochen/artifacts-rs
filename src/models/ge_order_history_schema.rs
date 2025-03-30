use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeOrderHistorySchema {
    /// Order id.
    #[serde(rename = "order_id")]
    pub order_id: String,
    /// Seller account name.
    #[serde(rename = "seller")]
    pub seller: String,
    /// Buyer account name.
    #[serde(rename = "buyer")]
    pub buyer: String,
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
    /// Item price per unit.
    #[serde(rename = "price")]
    pub price: u32,
    /// Sale datetime.
    #[serde(rename = "sold_at")]
    pub sold_at: String,
}

impl GeOrderHistorySchema {
    pub fn new(
        order_id: String,
        seller: String,
        buyer: String,
        code: String,
        quantity: u32,
        price: u32,
        sold_at: String,
    ) -> GeOrderHistorySchema {
        GeOrderHistorySchema {
            order_id,
            seller,
            buyer,
            code,
            quantity,
            price,
            sold_at,
        }
    }
}
