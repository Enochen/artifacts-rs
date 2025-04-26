use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UseItemSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Item details.
    #[serde(rename = "item")]
    pub item: Box<models::ItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl UseItemSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        item: models::ItemSchema,
        character: models::CharacterSchema,
    ) -> UseItemSchema {
        UseItemSchema {
            cooldown: Box::new(cooldown),
            item: Box::new(item),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for UseItemSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for UseItemSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
