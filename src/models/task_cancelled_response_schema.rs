use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TaskCancelledResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::TaskCancelledSchema>,
}

impl TaskCancelledResponseSchema {
    pub fn new(data: models::TaskCancelledSchema) -> TaskCancelledResponseSchema {
        TaskCancelledResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for TaskCancelledResponseSchema {
    type Data = Box<models::TaskCancelledSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
