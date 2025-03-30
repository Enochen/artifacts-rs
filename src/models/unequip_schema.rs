use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnequipSchema {
    /// Item slot.
    #[serde(rename = "slot")]
    pub slot: models::ItemSlot,
    /// Item quantity. Applicable to utilities only.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

impl UnequipSchema {
    pub fn new(slot: models::ItemSlot) -> UnequipSchema {
        UnequipSchema {
            slot,
            quantity: None,
        }
    }
}
