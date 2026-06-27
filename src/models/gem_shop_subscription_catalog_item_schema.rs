use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopSubscriptionCatalogItemSchema {
    /// Subscription offer code.
    #[serde(rename = "code")]
    pub code: String,
    /// Subscription offer name.
    #[serde(rename = "name")]
    pub name: String,
    /// Subscription duration in days.
    #[serde(rename = "duration_days")]
    pub duration_days: i32,
    /// Subscription price in gems.
    #[serde(rename = "price")]
    pub price: i32,
}

impl GemShopSubscriptionCatalogItemSchema {
    pub fn new(
        code: String,
        name: String,
        duration_days: i32,
        price: i32,
    ) -> GemShopSubscriptionCatalogItemSchema {
        GemShopSubscriptionCatalogItemSchema {
            code,
            name,
            duration_days,
            price,
        }
    }
}
