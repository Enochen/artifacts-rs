use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSkinCharacterSchema {
    /// Your desired skin. Skins unlocked by default: 'men1', 'men2', 'men3', 'women1', 'women2', 'women3'.
    #[serde(rename = "skin")]
    pub skin: models::CharacterSkin,
}

impl ChangeSkinCharacterSchema {
    pub fn new(skin: models::CharacterSkin) -> ChangeSkinCharacterSchema {
        ChangeSkinCharacterSchema { skin }
    }
}
