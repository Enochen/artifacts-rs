use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EffectSubtype {
    #[serde(rename = "stat")]
    Stat,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "heal")]
    Heal,
    #[serde(rename = "buff")]
    Buff,
    #[serde(rename = "debuff")]
    Debuff,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "gathering")]
    Gathering,
    #[serde(rename = "teleport")]
    Teleport,
    #[serde(rename = "gold")]
    Gold,
}

impl std::fmt::Display for EffectSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Stat => write!(f, "stat"),
            Self::Other => write!(f, "other"),
            Self::Heal => write!(f, "heal"),
            Self::Buff => write!(f, "buff"),
            Self::Debuff => write!(f, "debuff"),
            Self::Special => write!(f, "special"),
            Self::Gathering => write!(f, "gathering"),
            Self::Teleport => write!(f, "teleport"),
            Self::Gold => write!(f, "gold"),
        }
    }
}

impl Default for EffectSubtype {
    fn default() -> EffectSubtype {
        Self::Stat
    }
}
