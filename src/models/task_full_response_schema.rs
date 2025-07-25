use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct TaskFullResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::TaskFullSchema>,
}

impl TaskFullResponseSchema {
    pub fn new(data: models::TaskFullSchema) -> TaskFullResponseSchema {
        TaskFullResponseSchema {
            data: Box::new(data),
        }
    }
}

impl crate::traits::IntoData for TaskFullResponseSchema {
    type Data = Box<models::TaskFullSchema>;
    fn into_data(self) -> Self::Data {
        self.data
    }
}
