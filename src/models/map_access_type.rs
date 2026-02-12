use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum MapAccessType {
    #[serde(rename = "standard")]
    #[default]
    Standard,
    #[serde(rename = "teleportation")]
    Teleportation,
    #[serde(rename = "conditional")]
    Conditional,
    #[serde(rename = "blocked")]
    Blocked,
}

impl std::fmt::Display for MapAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Teleportation => write!(f, "teleportation"),
            Self::Conditional => write!(f, "conditional"),
            Self::Blocked => write!(f, "blocked"),
        }
    }
}
