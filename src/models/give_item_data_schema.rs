use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveItemDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Items given.
    #[serde(rename = "items")]
    pub items: Vec<models::SimpleItemSchema>,
    /// Character details of the receiving character.
    #[serde(rename = "receiver_character")]
    pub receiver_character: Box<models::CharacterSchema>,
    /// Character details of the sending character.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl GiveItemDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        items: Vec<models::SimpleItemSchema>,
        receiver_character: models::CharacterSchema,
        character: models::CharacterSchema,
    ) -> GiveItemDataSchema {
        GiveItemDataSchema {
            cooldown: Box::new(cooldown),
            items,
            receiver_character: Box::new(receiver_character),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for GiveItemDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for GiveItemDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
