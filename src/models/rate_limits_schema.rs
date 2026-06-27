use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RateLimitsSchema {
    #[serde(rename = "data")]
    pub data: Box<models::RateLimitsDataSchema>,
}

impl RateLimitsSchema {
    pub fn new(data: models::RateLimitsDataSchema) -> RateLimitsSchema {
        RateLimitsSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for RateLimitsSchema {
    type Data = Box<models::RateLimitsDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
