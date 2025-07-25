use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DeleteItemSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Item details.
    #[serde(rename = "item")]
    pub item: Box<models::SimpleItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl DeleteItemSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        item: models::SimpleItemSchema,
        character: models::CharacterSchema,
    ) -> DeleteItemSchema {
        DeleteItemSchema {
            cooldown: Box::new(cooldown),
            item: Box::new(item),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for DeleteItemSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for DeleteItemSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
