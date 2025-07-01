use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveGoldDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Quantity of gold given.
    #[serde(rename = "quantity")]
    pub quantity: u32,
    /// Character details of the receiving character.
    #[serde(rename = "receiver_character")]
    pub receiver_character: Box<models::CharacterSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl GiveGoldDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        quantity: u32,
        receiver_character: models::CharacterSchema,
        character: models::CharacterSchema,
    ) -> GiveGoldDataSchema {
        GiveGoldDataSchema {
            cooldown: Box::new(cooldown),
            quantity,
            receiver_character: Box::new(receiver_character),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for GiveGoldDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for GiveGoldDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
