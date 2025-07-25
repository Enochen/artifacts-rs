use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CraftingSchema {
    /// Craft code.
    #[serde(rename = "code")]
    pub code: String,
    /// Quantity of items to craft.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

impl CraftingSchema {
    pub fn new(code: String) -> CraftingSchema {
        CraftingSchema {
            code,
            quantity: None,
        }
    }
}
