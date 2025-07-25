use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct UseItemResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::UseItemSchema>,
}

impl UseItemResponseSchema {
    pub fn new(data: models::UseItemSchema) -> UseItemResponseSchema {
        UseItemResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for UseItemResponseSchema {
    type Data = Box<models::UseItemSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
