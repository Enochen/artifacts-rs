use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BankExtensionSchema {
    /// Price of the bank extension.
    #[serde(rename = "price")]
    pub price: u32,
}

impl BankExtensionSchema {
    pub fn new(price: u32) -> BankExtensionSchema {
        BankExtensionSchema { price }
    }
}
