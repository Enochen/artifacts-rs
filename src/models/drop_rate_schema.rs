use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DropRateSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Chance rate. (1/rate)
    #[serde(rename = "rate")]
    pub rate: u32,
    /// Minimum quantity.
    #[serde(rename = "min_quantity")]
    pub min_quantity: u32,
    /// Maximum quantity.
    #[serde(rename = "max_quantity")]
    pub max_quantity: u32,
}

impl DropRateSchema {
    pub fn new(code: String, rate: u32, min_quantity: u32, max_quantity: u32) -> DropRateSchema {
        DropRateSchema {
            code,
            rate,
            min_quantity,
            max_quantity,
        }
    }
}
