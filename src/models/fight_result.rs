use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum FightResult {
    #[serde(rename = "win")]
    #[default]
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
