use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeOrderCreatedSchema {
    /// Order id.
    #[serde(rename = "id")]
    pub id: String,
    /// Order created at.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
    /// Item price per unit.
    #[serde(rename = "price")]
    pub price: u32,
    /// Total price.
    #[serde(rename = "total_price")]
    pub total_price: u32,
    /// Listing tax (3%, minimum 1)
    #[serde(rename = "tax")]
    pub tax: u32,
}

impl GeOrderCreatedSchema {
    pub fn new(
        id: String,
        created_at: String,
        code: String,
        quantity: u32,
        price: u32,
        total_price: u32,
        tax: u32,
    ) -> GeOrderCreatedSchema {
        GeOrderCreatedSchema {
            id,
            created_at,
            code,
            quantity,
            price,
            total_price,
            tax,
        }
    }
}
