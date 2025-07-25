use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SkillDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Craft details.
    #[serde(rename = "details")]
    pub details: Box<models::SkillInfoSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl SkillDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        details: models::SkillInfoSchema,
        character: models::CharacterSchema,
    ) -> SkillDataSchema {
        SkillDataSchema {
            cooldown: Box::new(cooldown),
            details: Box::new(details),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for SkillDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for SkillDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
