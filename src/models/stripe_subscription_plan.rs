use crate::models;
use serde::{Deserialize, Serialize};

/// StripeSubscriptionPlan : Stripe subscription plan type.
/// Stripe subscription plan type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum StripeSubscriptionPlan {
    #[serde(rename = "monthly")]
    #[default]
    Monthly,
    #[serde(rename = "annual")]
    Annual,
}

impl std::fmt::Display for StripeSubscriptionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::Annual => write!(f, "annual"),
        }
    }
}
