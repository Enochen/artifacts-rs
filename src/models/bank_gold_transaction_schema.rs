use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankGoldTransactionSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Bank details.
    #[serde(rename = "bank")]
    pub bank: Box<models::GoldSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl BankGoldTransactionSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        bank: models::GoldSchema,
        character: models::CharacterSchema,
    ) -> BankGoldTransactionSchema {
        BankGoldTransactionSchema {
            cooldown: Box::new(cooldown),
            bank: Box::new(bank),
            character: Box::new(character),
        }
    }
}
