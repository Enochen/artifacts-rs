use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeOrderResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeOrderSchema>,
}

impl GeOrderResponseSchema {
    pub fn new(data: models::GeOrderSchema) -> GeOrderResponseSchema {
        GeOrderResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GeOrderResponseSchema {
    type Data = Box<models::GeOrderSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
