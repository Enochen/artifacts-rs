use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AcceptOfferingDataSchema {
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    #[serde(rename = "offering")]
    pub offering: Box<models::SeasonalOfferingSchema>,
    #[serde(rename = "favor_expires_at")]
    pub favor_expires_at: String,
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl AcceptOfferingDataSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        offering: models::SeasonalOfferingSchema,
        favor_expires_at: String,
        character: models::CharacterSchema,
    ) -> AcceptOfferingDataSchema {
        AcceptOfferingDataSchema {
            cooldown: Box::new(cooldown),
            offering: Box::new(offering),
            favor_expires_at,
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for AcceptOfferingDataSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for AcceptOfferingDataSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
