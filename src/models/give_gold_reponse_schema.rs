use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct GiveGoldReponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GiveGoldDataSchema>,
}

impl GiveGoldReponseSchema {
    pub fn new(data: models::GiveGoldDataSchema) -> GiveGoldReponseSchema {
        GiveGoldReponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GiveGoldReponseSchema {
    type Data = Box<models::GiveGoldDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
