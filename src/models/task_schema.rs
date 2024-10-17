use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSchema {
    /// Task objective.
    #[serde(rename = "code")]
    pub code: String,
    /// The type of task.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The total required to complete the task.
    #[serde(rename = "total")]
    pub total: i32,
}

impl TaskSchema {
    pub fn new(code: String, r#type: Type, total: i32) -> TaskSchema {
        TaskSchema {
            code,
            r#type,
            total,
        }
    }
}
/// The type of task.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "monsters")]
    Monsters,
    #[serde(rename = "items")]
    Items,
}

impl Default for Type {
    fn default() -> Type {
        Self::Monsters
    }
}
