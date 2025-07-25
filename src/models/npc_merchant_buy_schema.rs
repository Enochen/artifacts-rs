use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NpcMerchantBuySchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

impl NpcMerchantBuySchema {
    pub fn new(code: String, quantity: u32) -> NpcMerchantBuySchema {
        NpcMerchantBuySchema { code, quantity }
    }
}
