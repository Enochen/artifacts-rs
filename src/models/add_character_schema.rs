use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AddCharacterSchema {
    /// Your desired character name. It's unique and all players can see it.
    #[serde(rename = "name")]
    pub name: String,
    /// Your desired skin. Skins unlocked by default: 'men1', 'men2', 'men3', 'women1', 'women2', 'women3'.
    #[serde(rename = "skin")]
    pub skin: models::CharacterSkin,
}

impl AddCharacterSchema {
    pub fn new(name: String, skin: models::CharacterSkin) -> AddCharacterSchema {
        AddCharacterSchema { name, skin }
    }
}
