use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RewardDataResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::RewardDataSchema>,
}

impl RewardDataResponseSchema {
    pub fn new(data: models::RewardDataSchema) -> RewardDataResponseSchema {
        RewardDataResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for RewardDataResponseSchema {
    type Data = Box<models::RewardDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
