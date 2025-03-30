use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyAccountDetailsSchema {
    #[serde(rename = "data")]
    pub data: Box<models::MyAccountDetails>,
}

impl MyAccountDetailsSchema {
    pub fn new(data: models::MyAccountDetails) -> MyAccountDetailsSchema {
        MyAccountDetailsSchema {
            data: Box::new(data),
        }
    }
}
