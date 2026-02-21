use crate::models;
use serde::{Deserialize, Serialize};

/// GeOrderType : Type of Grand Exchange order.
/// Type of Grand Exchange order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum GeOrderType {
    #[serde(rename = "sell")]
    #[default]
    Sell,
    #[serde(rename = "buy")]
    Buy,
}

impl std::fmt::Display for GeOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Sell => write!(f, "sell"),
            Self::Buy => write!(f, "buy"),
        }
    }
}
