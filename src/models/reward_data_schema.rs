use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RewardDataSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Reward details.
    #[serde(rename = "rewards")]
    pub rewards: Box<models::RewardsSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl RewardDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        rewards: models::RewardsSchema,
        character: models::CharacterSchema,
    ) -> RewardDataSchema {
        RewardDataSchema {
            cooldown: Box::new(cooldown),
            rewards: Box::new(rewards),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for RewardDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for RewardDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
