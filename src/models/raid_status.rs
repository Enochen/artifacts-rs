use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum RaidStatus {
    #[serde(rename = "upcoming")]
    #[default]
    Upcoming,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "finished_success")]
    FinishedSuccess,
    #[serde(rename = "finished_failure")]
    FinishedFailure,
}

impl std::fmt::Display for RaidStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Upcoming => write!(f, "upcoming"),
            Self::Active => write!(f, "active"),
            Self::FinishedSuccess => write!(f, "finished_success"),
            Self::FinishedFailure => write!(f, "finished_failure"),
        }
    }
}
