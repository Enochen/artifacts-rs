use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TaskCancelledSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl TaskCancelledSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        character: models::CharacterSchema,
    ) -> TaskCancelledSchema {
        TaskCancelledSchema {
            cooldown: Box::new(cooldown),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for TaskCancelledSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for TaskCancelledSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
