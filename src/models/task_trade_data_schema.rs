use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TaskTradeDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Reward details.
    #[serde(rename = "trade")]
    pub trade: Box<models::TaskTradeSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl TaskTradeDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        trade: models::TaskTradeSchema,
        character: models::CharacterSchema,
    ) -> TaskTradeDataSchema {
        TaskTradeDataSchema {
            cooldown: Box::new(cooldown),
            trade: Box::new(trade),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for TaskTradeDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for TaskTradeDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
