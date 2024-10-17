use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GoldSchema {
    /// Quantity of gold.
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

impl GoldSchema {
    pub fn new(quantity: u32) -> GoldSchema {
        GoldSchema { quantity }
    }
}
