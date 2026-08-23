use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RenameCharacterSchema {
    /// Your desired character name. It's unique and all players can see it.
    #[serde(rename = "name")]
    pub name: String,
}

impl RenameCharacterSchema {
    pub fn new(name: String) -> RenameCharacterSchema {
        RenameCharacterSchema { name }
    }
}
