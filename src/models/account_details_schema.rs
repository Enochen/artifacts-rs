use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
