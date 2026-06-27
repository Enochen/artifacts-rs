use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopCustomDesignPurchaseResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GemShopCustomDesignPurchaseResponseDataSchema>,
}

impl GemShopCustomDesignPurchaseResponseSchema {
    pub fn new(
        data: models::GemShopCustomDesignPurchaseResponseDataSchema,
    ) -> GemShopCustomDesignPurchaseResponseSchema {
        GemShopCustomDesignPurchaseResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GemShopCustomDesignPurchaseResponseSchema {
    type Data = Box<models::GemShopCustomDesignPurchaseResponseDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
