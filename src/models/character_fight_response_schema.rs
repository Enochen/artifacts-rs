use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterFightResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CharacterFightDataSchema>,
}

impl CharacterFightResponseSchema {
    pub fn new(data: models::CharacterFightDataSchema) -> CharacterFightResponseSchema {
        CharacterFightResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for CharacterFightResponseSchema {
    type Data = Box<models::CharacterFightDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
