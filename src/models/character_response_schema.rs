use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CharacterSchema>,
}

impl CharacterResponseSchema {
    pub fn new(data: models::CharacterSchema) -> CharacterResponseSchema {
        CharacterResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::GetCharacter for CharacterResponseSchema {
    fn get_character(&self) -> &crate::models::CharacterSchema {
        &self.data
    }
}

impl crate::traits::IntoData for CharacterResponseSchema {
    type Data = Box<models::CharacterSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
