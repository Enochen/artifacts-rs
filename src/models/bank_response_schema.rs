use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct BankResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BankSchema>,
}

impl BankResponseSchema {
    pub fn new(data: models::BankSchema) -> BankResponseSchema {
        BankResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for BankResponseSchema {
    type Data = Box<models::BankSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
