use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
