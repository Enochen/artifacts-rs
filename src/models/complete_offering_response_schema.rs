use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CompleteOfferingResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CompleteOfferingDataSchema>,
}

impl CompleteOfferingResponseSchema {
    pub fn new(data: models::CompleteOfferingDataSchema) -> CompleteOfferingResponseSchema {
        CompleteOfferingResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for CompleteOfferingResponseSchema {
    type Data = Box<models::CompleteOfferingDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
