use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct EquipmentTransactionSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Items details.
    #[serde(rename = "items")]
    pub items: Vec<models::EquipmentItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl EquipmentTransactionSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        items: Vec<models::EquipmentItemSchema>,
        character: models::CharacterSchema,
    ) -> EquipmentTransactionSchema {
        EquipmentTransactionSchema {
            cooldown: Box::new(cooldown),
            items,
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for EquipmentTransactionSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for EquipmentTransactionSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
