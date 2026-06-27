use crate::models;
use serde::{Deserialize, Serialize};

/// GeOrderCreationSchema : Schema for creating a sell order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeOrderCreationSchema {
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

impl GeOrderCreationSchema {
    /// Schema for creating a sell order.
    pub fn new(code: String, quantity: u32, price: u32) -> GeOrderCreationSchema {
        GeOrderCreationSchema {
            code,
            quantity,
            price,
        }
    }
}
