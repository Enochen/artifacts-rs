use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GiveItemResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GiveItemDataSchema>,
}

impl GiveItemResponseSchema {
    pub fn new(data: models::GiveItemDataSchema) -> GiveItemResponseSchema {
        GiveItemResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GiveItemResponseSchema {
    type Data = Box<models::GiveItemDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
