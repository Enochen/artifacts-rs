use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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

impl crate::traits::IntoData for CharacterMovementResponseSchema {
    type Data = Box<models::CharacterMovementDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
