use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct InventorySlotSchema {
    /// Inventory slot identifier.
    #[serde(rename = "slot")]
    pub slot: i32,
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Quantity in the slot.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl InventorySlotSchema {
    pub fn new(slot: i32, code: String, quantity: i32) -> InventorySlotSchema {
        InventorySlotSchema {
            slot,
            code,
            quantity,
        }
    }
}
