use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EquipRequestSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Item slot.
    #[serde(rename = "slot")]
    pub slot: models::ItemSlot,
    /// Item details.
    #[serde(rename = "item")]
    pub item: Box<models::ItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl EquipRequestSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        slot: models::ItemSlot,
        item: models::ItemSchema,
        character: models::CharacterSchema,
    ) -> EquipRequestSchema {
        EquipRequestSchema {
            cooldown: Box::new(cooldown),
            slot,
            item: Box::new(item),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for EquipRequestSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for EquipRequestSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
