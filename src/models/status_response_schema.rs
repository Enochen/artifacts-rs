use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct StatusResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::StatusSchema>,
}

impl StatusResponseSchema {
    pub fn new(data: models::StatusSchema) -> StatusResponseSchema {
        StatusResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for StatusResponseSchema {
    type Data = Box<models::StatusSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
