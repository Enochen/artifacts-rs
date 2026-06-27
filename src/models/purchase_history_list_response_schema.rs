use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PurchaseHistoryListResponseSchema {
    #[serde(rename = "data")]
    pub data: Vec<models::PurchaseHistorySchema>,
}

impl PurchaseHistoryListResponseSchema {
    pub fn new(data: Vec<models::PurchaseHistorySchema>) -> PurchaseHistoryListResponseSchema {
        PurchaseHistoryListResponseSchema { data }
    }
}

impl crate::traits::IntoData for PurchaseHistoryListResponseSchema {
    type Data = Vec<models::PurchaseHistorySchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
