use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
