use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopCatalogDataSchema {
    /// Gem-shop skins.
    #[serde(rename = "skins")]
    pub skins: Vec<models::GemShopSkinCatalogItemSchema>,
    /// Gem-shop event spawns.
    #[serde(rename = "spawn_events")]
    pub spawn_events: Vec<models::GemShopSpawnEventCatalogItemSchema>,
    /// Gem-shop subscriptions.
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<models::GemShopSubscriptionCatalogItemSchema>,
    /// Gem-shop custom designs.
    #[serde(rename = "custom_designs")]
    pub custom_designs: Vec<models::GemShopCustomDesignCatalogItemSchema>,
}

impl GemShopCatalogDataSchema {
    pub fn new(
        skins: Vec<models::GemShopSkinCatalogItemSchema>,
        spawn_events: Vec<models::GemShopSpawnEventCatalogItemSchema>,
        subscriptions: Vec<models::GemShopSubscriptionCatalogItemSchema>,
        custom_designs: Vec<models::GemShopCustomDesignCatalogItemSchema>,
    ) -> GemShopCatalogDataSchema {
        GemShopCatalogDataSchema {
            skins,
            spawn_events,
            subscriptions,
            custom_designs,
        }
    }
}
