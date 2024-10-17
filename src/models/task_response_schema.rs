use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::TaskDataSchema>,
}

impl TaskResponseSchema {
    pub fn new(data: models::TaskDataSchema) -> TaskResponseSchema {
        TaskResponseSchema {
            data: Box::new(data),
        }
    }
}
