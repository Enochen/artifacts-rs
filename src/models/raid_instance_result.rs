use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum RaidInstanceResult {
    #[serde(rename = "success")]
    #[default]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

impl std::fmt::Display for RaidInstanceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failure => write!(f, "failure"),
        }
    }
}
