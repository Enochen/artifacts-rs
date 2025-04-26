use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecyclingResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::RecyclingDataSchema>,
}

impl RecyclingResponseSchema {
    pub fn new(data: models::RecyclingDataSchema) -> RecyclingResponseSchema {
        RecyclingResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for RecyclingResponseSchema {
    type Data = Box<models::RecyclingDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
