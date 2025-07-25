use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NpcMerchantTransactionSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Transaction details.
    #[serde(rename = "transaction")]
    pub transaction: Box<models::NpcItemTransactionSchema>,
    /// Character details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl NpcMerchantTransactionSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        transaction: models::NpcItemTransactionSchema,
        character: models::CharacterSchema,
    ) -> NpcMerchantTransactionSchema {
        NpcMerchantTransactionSchema {
            cooldown: Box::new(cooldown),
            transaction: Box::new(transaction),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for NpcMerchantTransactionSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for NpcMerchantTransactionSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
