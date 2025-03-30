use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeOrderReponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeOrderSchema>,
}

impl GeOrderReponseSchema {
    pub fn new(data: models::GeOrderSchema) -> GeOrderReponseSchema {
        GeOrderReponseSchema {
            data: Box::new(data),
        }
    }
}
