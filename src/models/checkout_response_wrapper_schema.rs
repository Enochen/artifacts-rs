use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CheckoutResponseWrapperSchema {
    #[serde(rename = "data")]
    pub data: Box<models::CheckoutResponseSchema>,
}

impl CheckoutResponseWrapperSchema {
    pub fn new(data: models::CheckoutResponseSchema) -> CheckoutResponseWrapperSchema {
        CheckoutResponseWrapperSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for CheckoutResponseWrapperSchema {
    type Data = Box<models::CheckoutResponseSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
