use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EquipSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item slot.
    #[serde(rename = "slot")]
    pub slot: models::ItemSlot,
    /// Item quantity. Applicable to utilities only.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

impl EquipSchema {
    pub fn new(code: String, slot: models::ItemSlot) -> EquipSchema {
        EquipSchema {
            code,
            slot,
            quantity: None,
        }
    }
}
