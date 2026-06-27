use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SubscriptionSchema {
    /// Subscription plan (monthly, annual, or prepaid).
    #[serde(rename = "plan")]
    pub plan: models::SubscriptionPlan,
    /// How the subscription was purchased. Mixed means both gems and member tokens were used.
    #[serde(rename = "purchase_source")]
    pub purchase_source: PurchaseSource,
    /// Subscription status (active, cancelled, past_due, expired).
    #[serde(rename = "status")]
    pub status: String,
    /// Start of the current billing period.
    #[serde(rename = "current_period_start")]
    pub current_period_start: String,
    /// End of the current billing period.
    #[serde(rename = "current_period_end")]
    pub current_period_end: String,
    /// When the subscription was created.
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// When the subscription was cancelled.
    #[serde(rename = "cancelled_at", skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<String>,
}

impl SubscriptionSchema {
    pub fn new(
        plan: models::SubscriptionPlan,
        purchase_source: PurchaseSource,
        status: String,
        current_period_start: String,
        current_period_end: String,
        created_at: String,
    ) -> SubscriptionSchema {
        SubscriptionSchema {
            plan,
            purchase_source,
            status,
            current_period_start,
            current_period_end,
            created_at,
            cancelled_at: None,
        }
    }
}
/// How the subscription was purchased. Mixed means both gems and member tokens were used.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[derive(Default)]
pub enum PurchaseSource {
    #[serde(rename = "stripe")]
    #[default]
    Stripe,
    #[serde(rename = "gems")]
    Gems,
    #[serde(rename = "member_token")]
    MemberToken,
    #[serde(rename = "mixed")]
    Mixed,
}
