use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeCreateOrderTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeOrderTransactionSchema>,
}

impl GeCreateOrderTransactionResponseSchema {
    pub fn new(data: models::GeOrderTransactionSchema) -> GeCreateOrderTransactionResponseSchema {
        GeCreateOrderTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GeCreateOrderTransactionResponseSchema {
    type Data = Box<models::GeOrderTransactionSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
