use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddCharacterSchema {
    /// Your desired character name. It's unique and all players can see it.
    #[serde(rename = "name")]
    pub name: String,
    /// Your desired skin.
    #[serde(rename = "skin")]
    pub skin: Skin,
}

impl AddCharacterSchema {
    pub fn new(name: String, skin: Skin) -> AddCharacterSchema {
        AddCharacterSchema { name, skin }
    }
}
/// Your desired skin.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Skin {
    #[serde(rename = "men1")]
    Men1,
    #[serde(rename = "men2")]
    Men2,
    #[serde(rename = "men3")]
    Men3,
    #[serde(rename = "women1")]
    Women1,
    #[serde(rename = "women2")]
    Women2,
    #[serde(rename = "women3")]
    Women3,
}

impl Default for Skin {
    fn default() -> Skin {
        Self::Men1
    }
}
