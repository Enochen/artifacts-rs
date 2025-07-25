use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct AccountDetailsSchema {
    #[serde(rename = "data")]
    pub data: Box<models::AccountDetails>,
}

impl AccountDetailsSchema {
    pub fn new(data: models::AccountDetails) -> AccountDetailsSchema {
        AccountDetailsSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for AccountDetailsSchema {
    type Data = Box<models::AccountDetails>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
