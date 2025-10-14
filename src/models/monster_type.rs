use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum MonsterType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "elite")]
    Elite,
    #[serde(rename = "boss")]
    Boss,
}

impl std::fmt::Display for MonsterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Elite => write!(f, "elite"),
            Self::Boss => write!(f, "boss"),
        }
    }
}

impl Default for MonsterType {
    fn default() -> MonsterType {
        Self::Normal
    }
}
