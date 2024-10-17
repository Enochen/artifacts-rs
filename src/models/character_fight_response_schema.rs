use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
