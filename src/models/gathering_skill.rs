use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GatheringSkill {
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "woodcutting")]
    Woodcutting,
    #[serde(rename = "fishing")]
    Fishing,
    #[serde(rename = "alchemy")]
    Alchemy,
}

impl std::fmt::Display for GatheringSkill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mining => write!(f, "mining"),
            Self::Woodcutting => write!(f, "woodcutting"),
            Self::Fishing => write!(f, "fishing"),
            Self::Alchemy => write!(f, "alchemy"),
        }
    }
}

impl Default for GatheringSkill {
    fn default() -> GatheringSkill {
        Self::Mining
    }
}
