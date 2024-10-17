use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
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
