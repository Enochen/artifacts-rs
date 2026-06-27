use crate::models;
use serde::{Deserialize, Serialize};

/// SubscriptionPlan : Subscription plan type.
/// Subscription plan type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum SubscriptionPlan {
    #[serde(rename = "monthly")]
    #[default]
    Monthly,
    #[serde(rename = "annual")]
    Annual,
    #[serde(rename = "prepaid")]
    Prepaid,
}

impl std::fmt::Display for SubscriptionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Annual => write!(f, "annual"),
            Self::Prepaid => write!(f, "prepaid"),
        }
    }
}
