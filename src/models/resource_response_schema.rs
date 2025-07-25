use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ResourceResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::ResourceSchema>,
}

impl ResourceResponseSchema {
    pub fn new(data: models::ResourceSchema) -> ResourceResponseSchema {
        ResourceResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for ResourceResponseSchema {
    type Data = Box<models::ResourceSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
