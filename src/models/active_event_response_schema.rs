use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct ActiveEventResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::ActiveEventSchema>,
}

impl ActiveEventResponseSchema {
    pub fn new(data: models::ActiveEventSchema) -> ActiveEventResponseSchema {
        ActiveEventResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for ActiveEventResponseSchema {
    type Data = Box<models::ActiveEventSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
