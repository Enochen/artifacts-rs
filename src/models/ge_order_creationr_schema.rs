use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeOrderCreationrSchema {
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

impl GeOrderCreationrSchema {
    pub fn new(code: String, quantity: u32, price: u32) -> GeOrderCreationrSchema {
        GeOrderCreationrSchema {
            code,
            quantity,
            price,
        }
    }
}
