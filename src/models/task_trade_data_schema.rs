use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
