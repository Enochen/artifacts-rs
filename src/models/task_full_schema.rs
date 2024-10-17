use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskFullSchema {
    /// Task objective.
    #[serde(rename = "code")]
    pub code: String,
    /// Task level.
    #[serde(rename = "level")]
    pub level: i32,
    /// The type of task.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Minimum amount of task.
    #[serde(rename = "min_quantity")]
    pub min_quantity: i32,
    /// Maximum amount of task.
    #[serde(rename = "max_quantity")]
    pub max_quantity: i32,
    #[serde(rename = "skill", deserialize_with = "Option::deserialize")]
    pub skill: Option<String>,
}

impl TaskFullSchema {
    pub fn new(
        code: String,
        level: i32,
        r#type: Type,
        min_quantity: i32,
        max_quantity: i32,
        skill: Option<String>,
    ) -> TaskFullSchema {
        TaskFullSchema {
            code,
            level,
            r#type,
            min_quantity,
            max_quantity,
            skill,
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
