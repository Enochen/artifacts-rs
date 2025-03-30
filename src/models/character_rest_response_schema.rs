use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterRestResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CharacterRestDataSchema>,
}

impl CharacterRestResponseSchema {
    pub fn new(data: models::CharacterRestDataSchema) -> CharacterRestResponseSchema {
        CharacterRestResponseSchema {
            data: Box::new(data),
        }
    }
}
