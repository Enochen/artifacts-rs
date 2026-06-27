use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PurchaseGemsRequestSchema {
    /// Number of gems to purchase.
    #[serde(rename = "quantity")]
    pub quantity: Quantity,
}

impl PurchaseGemsRequestSchema {
    pub fn new(quantity: Quantity) -> PurchaseGemsRequestSchema {
        PurchaseGemsRequestSchema { quantity }
    }
}
/// Number of gems to purchase.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum Quantity {
    #[serde(rename = "500")]
    #[default]
    Variant500,
    #[serde(rename = "1100")]
    Variant1100,
    #[serde(rename = "2400")]
    Variant2400,
    #[serde(rename = "6125")]
    Variant6125,
    #[serde(rename = "12500")]
    Variant12500,
}
