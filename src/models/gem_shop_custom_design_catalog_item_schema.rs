use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopCustomDesignCatalogItemSchema {
    /// Custom design code.
    #[serde(rename = "code")]
    pub code: String,
    /// Custom design name.
    #[serde(rename = "name")]
    pub name: String,
    /// Custom design description.
    #[serde(rename = "description")]
    pub description: String,
    /// Custom design price in gems.
    #[serde(rename = "price")]
    pub price: i32,
    /// Custom design category.
    #[serde(rename = "category")]
    pub category: Category,
    /// Whether the resulting content is unique to the buyer.
    #[serde(rename = "unique_to_account")]
    pub unique_to_account: bool,
}

impl GemShopCustomDesignCatalogItemSchema {
    pub fn new(
        code: String,
        name: String,
        description: String,
        price: i32,
        category: Category,
        unique_to_account: bool,
    ) -> GemShopCustomDesignCatalogItemSchema {
        GemShopCustomDesignCatalogItemSchema {
            code,
            name,
            description,
            price,
            category,
            unique_to_account,
        }
    }
}
/// Custom design category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum Category {
    #[serde(rename = "npc")]
    #[default]
    Npc,
    #[serde(rename = "item")]
    Item,
    #[serde(rename = "skin")]
    Skin,
}
