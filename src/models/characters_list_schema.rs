use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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

impl crate::traits::IntoData for CharactersListSchema {
    type Data = Vec<models::CharacterSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
