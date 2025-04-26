use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeTransactionListSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Transaction details.
    #[serde(rename = "order")]
    pub order: Box<models::GeTransactionSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl GeTransactionListSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        order: models::GeTransactionSchema,
        character: models::CharacterSchema,
    ) -> GeTransactionListSchema {
        GeTransactionListSchema {
            cooldown: Box::new(cooldown),
            order: Box::new(order),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for GeTransactionListSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for GeTransactionListSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
