use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SubscribeRequestSchema {
    /// Recurring Stripe subscription plan to purchase.
    #[serde(rename = "plan")]
    pub plan: models::StripeSubscriptionPlan,
}

impl SubscribeRequestSchema {
    pub fn new(plan: models::StripeSubscriptionPlan) -> SubscribeRequestSchema {
        SubscribeRequestSchema { plan }
    }
}
