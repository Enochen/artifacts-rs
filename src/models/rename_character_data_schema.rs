use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RenameCharacterDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Previous character name.
    #[serde(rename = "old_name")]
    pub old_name: String,
    /// New character name.
    #[serde(rename = "new_name")]
    pub new_name: String,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl RenameCharacterDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        old_name: String,
        new_name: String,
        character: models::CharacterSchema,
    ) -> RenameCharacterDataSchema {
        RenameCharacterDataSchema {
            cooldown: Box::new(cooldown),
            old_name,
            new_name,
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for RenameCharacterDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for RenameCharacterDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
