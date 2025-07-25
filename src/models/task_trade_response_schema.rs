use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TaskTradeResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::TaskTradeDataSchema>,
}

impl TaskTradeResponseSchema {
    pub fn new(data: models::TaskTradeDataSchema) -> TaskTradeResponseSchema {
        TaskTradeResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for TaskTradeResponseSchema {
    type Data = Box<models::TaskTradeDataSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
