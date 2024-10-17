use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipmentResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::EquipRequestSchema>,
}

impl EquipmentResponseSchema {
    pub fn new(data: models::EquipRequestSchema) -> EquipmentResponseSchema {
        EquipmentResponseSchema {
            data: Box::new(data),
        }
    }
}
