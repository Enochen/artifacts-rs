use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TasksRewardFullResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::TasksRewardFullSchema>,
}

impl TasksRewardFullResponseSchema {
    pub fn new(data: models::TasksRewardFullSchema) -> TasksRewardFullResponseSchema {
        TasksRewardFullResponseSchema {
            data: Box::new(data),
        }
    }
}
