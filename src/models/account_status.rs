use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum AccountStatus {
    #[serde(rename = "standard")]
    #[default]
    Standard,
    #[serde(rename = "founder")]
    Founder,
    #[serde(rename = "gold_founder")]
    GoldFounder,
    #[serde(rename = "vip_founder")]
    VipFounder,
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Founder => write!(f, "founder"),
            Self::GoldFounder => write!(f, "gold_founder"),
            Self::VipFounder => write!(f, "vip_founder"),
        }
    }
}
