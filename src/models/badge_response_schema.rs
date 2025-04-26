use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BadgeResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BadgeSchema>,
}

impl BadgeResponseSchema {
    pub fn new(data: models::BadgeSchema) -> BadgeResponseSchema {
        BadgeResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for BadgeResponseSchema {
    type Data = Box<models::BadgeSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
