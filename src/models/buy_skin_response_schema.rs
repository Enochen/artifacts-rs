use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BuySkinResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BuySkinResponseDataSchema>,
}

impl BuySkinResponseSchema {
    pub fn new(data: models::BuySkinResponseDataSchema) -> BuySkinResponseSchema {
        BuySkinResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for BuySkinResponseSchema {
    type Data = Box<models::BuySkinResponseDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
