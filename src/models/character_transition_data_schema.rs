use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterTransitionDataSchema {
    /// Cooldown details
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Destination map details.
    #[serde(rename = "destination")]
    pub destination: Box<models::MapSchema>,
    /// Transition details.
    #[serde(rename = "transition")]
    pub transition: Box<models::TransitionSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl CharacterTransitionDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        destination: models::MapSchema,
        transition: models::TransitionSchema,
        character: models::CharacterSchema,
    ) -> CharacterTransitionDataSchema {
        CharacterTransitionDataSchema {
            cooldown: Box::new(cooldown),
            destination: Box::new(destination),
            transition: Box::new(transition),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for CharacterTransitionDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for CharacterTransitionDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
