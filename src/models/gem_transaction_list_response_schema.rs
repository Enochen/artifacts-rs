use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemTransactionListResponseSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::GemTransactionSchema>,
}

impl GemTransactionListResponseSchema {
    pub fn new(data: Vec<models::GemTransactionSchema>) -> GemTransactionListResponseSchema {
        GemTransactionListResponseSchema { data }
    }
}

impl crate::traits::IntoData for GemTransactionListResponseSchema {
    type Data = Vec<models::GemTransactionSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
