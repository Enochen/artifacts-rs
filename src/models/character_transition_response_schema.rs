use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterTransitionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CharacterTransitionDataSchema>,
}

impl CharacterTransitionResponseSchema {
    pub fn new(data: models::CharacterTransitionDataSchema) -> CharacterTransitionResponseSchema {
        CharacterTransitionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for CharacterTransitionResponseSchema {
    type Data = Box<models::CharacterTransitionDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
