use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterMovementResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CharacterMovementDataSchema>,
}

impl CharacterMovementResponseSchema {
    pub fn new(data: models::CharacterMovementDataSchema) -> CharacterMovementResponseSchema {
        CharacterMovementResponseSchema {
            data: Box::new(data),
        }
    }
}
