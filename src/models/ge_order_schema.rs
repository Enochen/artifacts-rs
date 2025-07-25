use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeOrderSchema {
    /// Order id.
    #[serde(rename = "id")]
    pub id: String,
    /// Seller account name.
    #[serde(rename = "seller")]
    pub seller: String,
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
    /// Item price per unit.
    #[serde(rename = "price")]
    pub price: u32,
    /// Order created at.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl GeOrderSchema {
    pub fn new(
        id: String,
        seller: String,
        code: String,
        quantity: u32,
        price: u32,
        created_at: String,
    ) -> GeOrderSchema {
        GeOrderSchema {
            id,
            seller,
            code,
            quantity,
            price,
            created_at,
        }
    }
}
