use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GiveGoldResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GiveGoldDataSchema>,
}

impl GiveGoldResponseSchema {
    pub fn new(data: models::GiveGoldDataSchema) -> GiveGoldResponseSchema {
        GiveGoldResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GiveGoldResponseSchema {
    type Data = Box<models::GiveGoldDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
