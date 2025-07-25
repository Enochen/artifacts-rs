use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DeleteCharacterSchema {
    /// Character name.
    #[serde(rename = "name")]
    pub name: String,
}

impl DeleteCharacterSchema {
    pub fn new(name: String) -> DeleteCharacterSchema {
        DeleteCharacterSchema { name }
    }
}
