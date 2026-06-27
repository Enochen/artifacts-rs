use crate::models;
use serde::{Deserialize, Serialize};

/// PurchaseType : Type of purchase in history.
/// Type of purchase in history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum PurchaseType {
    #[serde(rename = "subscription")]
    #[default]
    Subscription,
    #[serde(rename = "gem_pack")]
    GemPack,
}

impl std::fmt::Display for PurchaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Subscription => write!(f, "subscription"),
            Self::GemPack => write!(f, "gem_pack"),
        }
    }
}
