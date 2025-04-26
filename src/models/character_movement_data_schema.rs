use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterMovementDataSchema {
    /// Cooldown details
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Destination details.
    #[serde(rename = "destination")]
    pub destination: Box<models::MapSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl CharacterMovementDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        destination: models::MapSchema,
        character: models::CharacterSchema,
    ) -> CharacterMovementDataSchema {
        CharacterMovementDataSchema {
            cooldown: Box::new(cooldown),
            destination: Box::new(destination),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for CharacterMovementDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for CharacterMovementDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
