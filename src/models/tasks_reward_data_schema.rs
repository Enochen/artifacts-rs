use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksRewardDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Reward details.
    #[serde(rename = "reward")]
    pub reward: Box<models::TasksRewardSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl TasksRewardDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        reward: models::TasksRewardSchema,
        character: models::CharacterSchema,
    ) -> TasksRewardDataSchema {
        TasksRewardDataSchema {
            cooldown: Box::new(cooldown),
            reward: Box::new(reward),
            character: Box::new(character),
        }
    }
}
