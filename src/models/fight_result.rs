use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum FightResult {
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "loss")]
    Loss,
}

impl std::fmt::Display for FightResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Win => write!(f, "win"),
            Self::Loss => write!(f, "loss"),
        }
    }
}

impl Default for FightResult {
    fn default() -> FightResult {
        Self::Win
    }
}
