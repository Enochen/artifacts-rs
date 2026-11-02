use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CompleteOfferingDataSchema {
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    #[serde(rename = "offering")]
    pub offering: Box<models::SeasonalOfferingSchema>,
    #[serde(rename = "reward")]
    pub reward: Box<models::OfferingRewardSchema>,
    #[serde(rename = "favor_expires_at")]
    pub favor_expires_at: String,
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl CompleteOfferingDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        offering: models::SeasonalOfferingSchema,
        reward: models::OfferingRewardSchema,
        favor_expires_at: String,
        character: models::CharacterSchema,
    ) -> CompleteOfferingDataSchema {
        CompleteOfferingDataSchema {
            cooldown: Box::new(cooldown),
            offering: Box::new(offering),
            reward: Box::new(reward),
            favor_expires_at,
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for CompleteOfferingDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for CompleteOfferingDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
