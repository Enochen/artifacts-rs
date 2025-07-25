use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GeTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeTransactionListSchema>,
}

impl GeTransactionResponseSchema {
    pub fn new(data: models::GeTransactionListSchema) -> GeTransactionResponseSchema {
        GeTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GeTransactionResponseSchema {
    type Data = Box<models::GeTransactionListSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
