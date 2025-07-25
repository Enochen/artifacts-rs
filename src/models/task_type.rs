use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum TaskType {
    #[serde(rename = "monsters")]
    Monsters,
    #[serde(rename = "items")]
    Items,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Monsters => write!(f, "monsters"),
            Self::Items => write!(f, "items"),
        }
    }
}

impl Default for TaskType {
    fn default() -> TaskType {
        Self::Monsters
    }
}
