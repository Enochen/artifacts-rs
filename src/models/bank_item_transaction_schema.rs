use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BankItemTransactionSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Items details.
    #[serde(rename = "items")]
    pub items: Vec<models::SimpleItemSchema>,
    /// Items in your banks.
    #[serde(rename = "bank")]
    pub bank: Vec<models::SimpleItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl BankItemTransactionSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        items: Vec<models::SimpleItemSchema>,
        bank: Vec<models::SimpleItemSchema>,
        character: models::CharacterSchema,
    ) -> BankItemTransactionSchema {
        BankItemTransactionSchema {
            cooldown: Box::new(cooldown),
            items,
            bank,
            character: Box::new(character),
        }
    }
}

impl crate::traits::GetCooldown for BankItemTransactionSchema {
    fn get_cooldown(&self) -> &crate::models::CooldownSchema {
        &self.cooldown
    }
}

impl crate::traits::GetCharacter for BankItemTransactionSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.character
    }
}
