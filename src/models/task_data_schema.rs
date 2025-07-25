use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TaskDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Task details.
    #[serde(rename = "task")]
    pub task: Box<models::TaskSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl TaskDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        task: models::TaskSchema,
        character: models::CharacterSchema,
    ) -> TaskDataSchema {
        TaskDataSchema {
            cooldown: Box::new(cooldown),
            task: Box::new(task),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for TaskDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for TaskDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
