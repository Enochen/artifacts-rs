use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SkinResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::SkinSchema>,
}

impl SkinResponseSchema {
    pub fn new(data: models::SkinSchema) -> SkinResponseSchema {
        SkinResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for SkinResponseSchema {
    type Data = Box<models::SkinSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
