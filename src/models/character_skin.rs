use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum CharacterSkin {
    #[serde(rename = "men1")]
    Men1,
    #[serde(rename = "men2")]
    Men2,
    #[serde(rename = "men3")]
    Men3,
    #[serde(rename = "women1")]
    Women1,
    #[serde(rename = "women2")]
    Women2,
    #[serde(rename = "women3")]
    Women3,
    #[serde(rename = "corrupted1")]
    Corrupted1,
    #[serde(rename = "zombie1")]
    Zombie1,
}

impl std::fmt::Display for CharacterSkin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Men1 => write!(f, "men1"),
            Self::Men2 => write!(f, "men2"),
            Self::Men3 => write!(f, "men3"),
            Self::Women1 => write!(f, "women1"),
            Self::Women2 => write!(f, "women2"),
            Self::Women3 => write!(f, "women3"),
            Self::Corrupted1 => write!(f, "corrupted1"),
            Self::Zombie1 => write!(f, "zombie1"),
        }
    }
}

impl Default for CharacterSkin {
    fn default() -> CharacterSkin {
        Self::Men1
    }
}
