use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GemShopCatalogResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GemShopCatalogDataSchema>,
}

impl GemShopCatalogResponseSchema {
    pub fn new(data: models::GemShopCatalogDataSchema) -> GemShopCatalogResponseSchema {
        GemShopCatalogResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GemShopCatalogResponseSchema {
    type Data = Box<models::GemShopCatalogDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
