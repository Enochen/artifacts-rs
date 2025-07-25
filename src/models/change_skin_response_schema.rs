use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ChangeSkinResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::ChangeSkinCharacterDataSchema>,
}

impl ChangeSkinResponseSchema {
    pub fn new(data: models::ChangeSkinCharacterDataSchema) -> ChangeSkinResponseSchema {
        ChangeSkinResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for ChangeSkinResponseSchema {
    type Data = Box<models::ChangeSkinCharacterDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
