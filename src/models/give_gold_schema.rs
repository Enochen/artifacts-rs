use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveGoldSchema {
    /// Gold quantity.
    #[serde(rename = "quantity")]
    pub quantity: u32,
    /// Character name. The name of the character who will receive the gold.
    #[serde(rename = "character")]
    pub character: String,
}

impl GiveGoldSchema {
    pub fn new(quantity: u32, character: String) -> GiveGoldSchema {
        GiveGoldSchema {
            quantity,
            character,
        }
    }
}
