use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksRewardResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::TasksRewardDataSchema>,
}

impl TasksRewardResponseSchema {
    pub fn new(data: models::TasksRewardDataSchema) -> TasksRewardResponseSchema {
        TasksRewardResponseSchema {
            data: Box::new(data),
        }
    }
}
