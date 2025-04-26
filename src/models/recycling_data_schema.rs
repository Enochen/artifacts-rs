use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecyclingDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Craft details.
    #[serde(rename = "details")]
    pub details: Box<models::RecyclingItemsSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl RecyclingDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        details: models::RecyclingItemsSchema,
        character: models::CharacterSchema,
    ) -> RecyclingDataSchema {
        RecyclingDataSchema {
            cooldown: Box::new(cooldown),
            details: Box::new(details),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for RecyclingDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for RecyclingDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
