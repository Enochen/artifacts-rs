use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RewardResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::DropRateSchema>,
}

impl RewardResponseSchema {
    pub fn new(data: models::DropRateSchema) -> RewardResponseSchema {
        RewardResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for RewardResponseSchema {
    type Data = Box<models::DropRateSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
