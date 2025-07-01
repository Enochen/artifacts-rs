use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveItemReponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GiveItemDataSchema>,
}

impl GiveItemReponseSchema {
    pub fn new(data: models::GiveItemDataSchema) -> GiveItemReponseSchema {
        GiveItemReponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for GiveItemReponseSchema {
    type Data = Box<models::GiveItemDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
