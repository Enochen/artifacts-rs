use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopSubscriptionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GemShopSubscriptionResponseDataSchema>,
}

impl GemShopSubscriptionResponseSchema {
    pub fn new(
        data: models::GemShopSubscriptionResponseDataSchema,
    ) -> GemShopSubscriptionResponseSchema {
        GemShopSubscriptionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GemShopSubscriptionResponseSchema {
    type Data = Box<models::GemShopSubscriptionResponseDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
