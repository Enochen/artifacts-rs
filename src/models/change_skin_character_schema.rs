use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChangeSkinCharacterSchema {
    /// Your desired skin.
    #[serde(rename = "skin")]
    pub skin: String,
}

impl ChangeSkinCharacterSchema {
    pub fn new(skin: String) -> ChangeSkinCharacterSchema {
        ChangeSkinCharacterSchema { skin }
    }
}
