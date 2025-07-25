use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum CharacterLeaderboardType {
    #[serde(rename = "combat")]
    Combat,
    #[serde(rename = "woodcutting")]
    Woodcutting,
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "fishing")]
    Fishing,
    #[serde(rename = "weaponcrafting")]
    Weaponcrafting,
    #[serde(rename = "gearcrafting")]
    Gearcrafting,
    #[serde(rename = "jewelrycrafting")]
    Jewelrycrafting,
    #[serde(rename = "cooking")]
    Cooking,
    #[serde(rename = "alchemy")]
    Alchemy,
}

impl std::fmt::Display for CharacterLeaderboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Combat => write!(f, "combat"),
            Self::Woodcutting => write!(f, "woodcutting"),
            Self::Mining => write!(f, "mining"),
            Self::Fishing => write!(f, "fishing"),
            Self::Weaponcrafting => write!(f, "weaponcrafting"),
            Self::Gearcrafting => write!(f, "gearcrafting"),
            Self::Jewelrycrafting => write!(f, "jewelrycrafting"),
            Self::Cooking => write!(f, "cooking"),
            Self::Alchemy => write!(f, "alchemy"),
        }
    }
}

impl Default for CharacterLeaderboardType {
    fn default() -> CharacterLeaderboardType {
        Self::Combat
    }
}
