use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NpcResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::NpcSchema>,
}

impl NpcResponseSchema {
    pub fn new(data: models::NpcSchema) -> NpcResponseSchema {
        NpcResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for NpcResponseSchema {
    type Data = Box<models::NpcSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
