use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum MapLayer {
    #[serde(rename = "interior")]
    Interior,
    #[serde(rename = "overworld")]
    Overworld,
    #[serde(rename = "underground")]
    Underground,
}

impl std::fmt::Display for MapLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Interior => write!(f, "interior"),
            Self::Overworld => write!(f, "overworld"),
            Self::Underground => write!(f, "underground"),
        }
    }
}

impl Default for MapLayer {
    fn default() -> MapLayer {
        Self::Interior
    }
}
