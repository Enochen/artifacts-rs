use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeOrderReponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeOrderSchema>,
}

impl GeOrderReponseSchema {
    pub fn new(data: models::GeOrderSchema) -> GeOrderReponseSchema {
        GeOrderReponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GeOrderReponseSchema {
    type Data = Box<models::GeOrderSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
