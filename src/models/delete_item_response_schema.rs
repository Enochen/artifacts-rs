use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteItemResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::DeleteItemSchema>,
}

impl DeleteItemResponseSchema {
    pub fn new(data: models::DeleteItemSchema) -> DeleteItemResponseSchema {
        DeleteItemResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for DeleteItemResponseSchema {
    type Data = Box<models::DeleteItemSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
