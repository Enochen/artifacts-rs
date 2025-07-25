use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BankExtensionTransactionSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Transaction details.
    #[serde(rename = "transaction")]
    pub transaction: Box<models::BankExtensionSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl BankExtensionTransactionSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        transaction: models::BankExtensionSchema,
        character: models::CharacterSchema,
    ) -> BankExtensionTransactionSchema {
        BankExtensionTransactionSchema {
            cooldown: Box::new(cooldown),
            transaction: Box::new(transaction),
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for BankExtensionTransactionSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for BankExtensionTransactionSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
