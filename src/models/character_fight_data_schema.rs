use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterFightDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Character fight details.
    #[serde(rename = "fight")]
    pub fight: Box<models::CharacterFightSchema>,
    /// All characters involved.
    #[serde(rename = "characters")]
    pub characters: Vec<models::CharacterSchema>,
}

impl CharacterFightDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        fight: models::CharacterFightSchema,
        characters: Vec<models::CharacterSchema>,
    ) -> CharacterFightDataSchema {
        CharacterFightDataSchema {
            cooldown: Box::new(cooldown),
            fight: Box::new(fight),
            characters,
        }
    }
}

impl crate::traits::GetCooldown for CharacterFightDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacters for CharacterFightDataSchema {
    fn get_characters(&self) -> Vec<crate::models::CharacterSchema> {
        self.characters.clone()
    }
}
