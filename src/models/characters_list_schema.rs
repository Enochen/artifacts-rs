use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharactersListSchema {
    /// List of your characters.
    #[serde(rename = "data")]
    pub data: Vec<models::CharacterSchema>,
}

impl CharactersListSchema {
    pub fn new(data: Vec<models::CharacterSchema>) -> CharactersListSchema {
        CharactersListSchema { data }
    }
}
