use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RenameResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::RenameCharacterDataSchema>,
}

impl RenameResponseSchema {
    pub fn new(data: models::RenameCharacterDataSchema) -> RenameResponseSchema {
        RenameResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for RenameResponseSchema {
    type Data = Box<models::RenameCharacterDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
