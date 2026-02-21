use crate::models;
use serde::{Deserialize, Serialize};

/// GeBuyOrderSchema : Schema for buying from a sell order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeBuyOrderSchema {
    /// Order id.
    #[serde(rename = "id")]
    pub id: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

impl GeBuyOrderSchema {
    /// Schema for buying from a sell order.
    pub fn new(id: String, quantity: u32) -> GeBuyOrderSchema {
        GeBuyOrderSchema { id, quantity }
    }
}
