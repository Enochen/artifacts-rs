use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
