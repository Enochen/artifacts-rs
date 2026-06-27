use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EquipmentItemSchema {
    /// Item slot.
    #[serde(rename = "slot")]
    pub slot: models::ItemSlot,
    /// Item details.
    #[serde(rename = "item")]
    pub item: Box<models::ItemSchema>,
}

impl EquipmentItemSchema {
    pub fn new(slot: models::ItemSlot, item: models::ItemSchema) -> EquipmentItemSchema {
        EquipmentItemSchema {
            slot,
            item: Box::new(item),
        }
    }
}
