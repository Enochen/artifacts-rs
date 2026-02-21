use crate::models;
use serde::{Deserialize, Serialize};

/// GeBuyOrderCreationSchema : Schema for creating a buy order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeBuyOrderCreationSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
    /// Item price per unit.
    #[serde(rename = "price")]
    pub price: u32,
}

impl GeBuyOrderCreationSchema {
    /// Schema for creating a buy order.
    pub fn new(code: String, quantity: u32, price: u32) -> GeBuyOrderCreationSchema {
        GeBuyOrderCreationSchema {
            code,
            quantity,
            price,
        }
    }
}
