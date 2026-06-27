use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CharacterStatsResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CharacterStatsSchema>,
}

impl CharacterStatsResponseSchema {
    pub fn new(data: models::CharacterStatsSchema) -> CharacterStatsResponseSchema {
        CharacterStatsResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for CharacterStatsResponseSchema {
    type Data = Box<models::CharacterStatsSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
