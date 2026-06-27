use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum MapContentType {
    #[serde(rename = "monster")]
    #[default]
    Monster,
    #[serde(rename = "resource")]
    Resource,
    #[serde(rename = "workshop")]
    Workshop,
    #[serde(rename = "bank")]
    Bank,
    #[serde(rename = "grand_exchange")]
    GrandExchange,
    #[serde(rename = "tasks_master")]
    TasksMaster,
    #[serde(rename = "npc")]
    Npc,
    #[serde(rename = "raid")]
    Raid,
}

impl std::fmt::Display for MapContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Monster => write!(f, "monster"),
            Self::Resource => write!(f, "resource"),
            Self::Workshop => write!(f, "workshop"),
            Self::Bank => write!(f, "bank"),
            Self::GrandExchange => write!(f, "grand_exchange"),
            Self::TasksMaster => write!(f, "tasks_master"),
            Self::Npc => write!(f, "npc"),
            Self::Raid => write!(f, "raid"),
        }
    }
}
