use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeOrderTransactionSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Order details.
    #[serde(rename = "order")]
    pub order: Box<models::GeOrderCreatedSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl GeOrderTransactionSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        order: models::GeOrderCreatedSchema,
        character: models::CharacterSchema,
    ) -> GeOrderTransactionSchema {
        GeOrderTransactionSchema {
            cooldown: Box::new(cooldown),
            order: Box::new(order),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for GeOrderTransactionSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for GeOrderTransactionSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
