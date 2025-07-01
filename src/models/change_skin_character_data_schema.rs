use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSkinCharacterDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Craft details.
    #[serde(rename = "skin")]
    pub skin: String,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl ChangeSkinCharacterDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        skin: String,
        character: models::CharacterSchema,
    ) -> ChangeSkinCharacterDataSchema {
        ChangeSkinCharacterDataSchema {
            cooldown: Box::new(cooldown),
            skin,
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for ChangeSkinCharacterDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for ChangeSkinCharacterDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
