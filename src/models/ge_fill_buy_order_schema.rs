use crate::models;
use serde::{Deserialize, Serialize};

/// GeFillBuyOrderSchema : Schema for filling a buy order (selling items to a buy order).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeFillBuyOrderSchema {
    /// Buy order id.
    #[serde(rename = "id")]
    pub id: String,
    /// Item quantity to sell.
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

impl GeFillBuyOrderSchema {
    /// Schema for filling a buy order (selling items to a buy order).
    pub fn new(id: String, quantity: u32) -> GeFillBuyOrderSchema {
        GeFillBuyOrderSchema { id, quantity }
    }
}
