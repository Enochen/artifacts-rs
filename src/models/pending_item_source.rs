use crate::models;
use serde::{Deserialize, Serialize};

/// PendingItemSource : Source types for pending items.
/// Source types for pending items.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum PendingItemSource {
    #[serde(rename = "achievement")]
    #[default]
    Achievement,
    #[serde(rename = "grand_exchange")]
    GrandExchange,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "raid")]
    Raid,
    #[serde(rename = "other")]
    Other,
}

impl std::fmt::Display for PendingItemSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Achievement => write!(f, "achievement"),
            Self::GrandExchange => write!(f, "grand_exchange"),
            Self::Admin => write!(f, "admin"),
            Self::Event => write!(f, "event"),
            Self::Raid => write!(f, "raid"),
            Self::Other => write!(f, "other"),
        }
    }
}
