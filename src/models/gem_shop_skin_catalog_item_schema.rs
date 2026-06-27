use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopSkinCatalogItemSchema {
    /// Skin code.
    #[serde(rename = "code")]
    pub code: String,
    /// Skin name.
    #[serde(rename = "name")]
    pub name: String,
    /// Skin description.
    #[serde(rename = "description")]
    pub description: String,
    /// Skin price in gems.
    #[serde(rename = "price")]
    pub price: i32,
}

impl GemShopSkinCatalogItemSchema {
    pub fn new(
        code: String,
        name: String,
        description: String,
        price: i32,
    ) -> GemShopSkinCatalogItemSchema {
        GemShopSkinCatalogItemSchema {
            code,
            name,
            description,
            price,
        }
    }
}
