use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AddCharacterSchema {
    /// Your desired character name. It's unique and all players can see it.
    #[serde(rename = "name")]
    pub name: String,
    /// Your desired skin.
    #[serde(rename = "skin")]
    pub skin: String,
}

impl AddCharacterSchema {
    pub fn new(name: String, skin: String) -> AddCharacterSchema {
        AddCharacterSchema { name, skin }
    }
}
