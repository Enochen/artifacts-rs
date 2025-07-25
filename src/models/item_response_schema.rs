use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ItemResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::ItemSchema>,
}

impl ItemResponseSchema {
    pub fn new(data: models::ItemSchema) -> ItemResponseSchema {
        ItemResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for ItemResponseSchema {
    type Data = Box<models::ItemSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
