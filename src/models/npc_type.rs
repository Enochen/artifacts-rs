use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum NpcType {
    #[serde(rename = "merchant")]
    #[default]
    Merchant,
    #[serde(rename = "trader")]
    Trader,
}

impl std::fmt::Display for NpcType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Merchant => write!(f, "merchant"),
            Self::Trader => write!(f, "trader"),
        }
    }
}
