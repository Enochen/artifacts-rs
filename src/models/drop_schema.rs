use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DropSchema {
    /// The code of the item.
    #[serde(rename = "code")]
    pub code: String,
    /// The quantity of the item.
    #[serde(rename = "quantity")]
    pub quantity: i32,
}

impl DropSchema {
    pub fn new(code: String, quantity: i32) -> DropSchema {
        DropSchema { code, quantity }
    }
}
