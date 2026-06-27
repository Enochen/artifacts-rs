use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SubscriptionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::SubscriptionSchema>,
}

impl SubscriptionResponseSchema {
    pub fn new(data: models::SubscriptionSchema) -> SubscriptionResponseSchema {
        SubscriptionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for SubscriptionResponseSchema {
    type Data = Box<models::SubscriptionSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
