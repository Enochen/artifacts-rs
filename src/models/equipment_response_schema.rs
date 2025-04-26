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

impl crate::traits::IntoData for EquipmentResponseSchema {
    type Data = Box<models::EquipRequestSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
