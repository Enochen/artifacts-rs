use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
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

impl crate::traits::IntoData for MyAccountDetailsSchema {
    type Data = Box<models::MyAccountDetails>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
