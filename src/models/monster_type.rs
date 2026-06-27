use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum MonsterType {
    #[serde(rename = "normal")]
    #[default]
    Normal,
    #[serde(rename = "elite")]
    Elite,
    #[serde(rename = "boss")]
    Boss,
    #[serde(rename = "raid_boss")]
    RaidBoss,
}

impl std::fmt::Display for MonsterType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Elite => write!(f, "elite"),
            Self::Boss => write!(f, "boss"),
            Self::RaidBoss => write!(f, "raid_boss"),
        }
    }
}
