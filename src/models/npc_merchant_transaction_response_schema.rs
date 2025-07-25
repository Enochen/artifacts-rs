use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct NpcMerchantTransactionResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::NpcMerchantTransactionSchema>,
}

impl NpcMerchantTransactionResponseSchema {
    pub fn new(data: models::NpcMerchantTransactionSchema) -> NpcMerchantTransactionResponseSchema {
        NpcMerchantTransactionResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for NpcMerchantTransactionResponseSchema {
    type Data = Box<models::NpcMerchantTransactionSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
