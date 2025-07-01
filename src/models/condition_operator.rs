use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConditionOperator {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "lt")]
    Lt,
}

impl std::fmt::Display for ConditionOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Ne => write!(f, "ne"),
            Self::Gt => write!(f, "gt"),
            Self::Lt => write!(f, "lt"),
        }
    }
}

impl Default for ConditionOperator {
    fn default() -> ConditionOperator {
        Self::Eq
    }
}
