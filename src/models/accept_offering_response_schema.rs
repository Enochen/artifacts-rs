use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AcceptOfferingResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::AcceptOfferingDataSchema>,
}

impl AcceptOfferingResponseSchema {
    pub fn new(data: models::AcceptOfferingDataSchema) -> AcceptOfferingResponseSchema {
        AcceptOfferingResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for AcceptOfferingResponseSchema {
    type Data = Box<models::AcceptOfferingDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
