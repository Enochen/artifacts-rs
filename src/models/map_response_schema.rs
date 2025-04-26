use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::MapSchema>,
}

impl MapResponseSchema {
    pub fn new(data: models::MapSchema) -> MapResponseSchema {
        MapResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for MapResponseSchema {
    type Data = Box<models::MapSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
