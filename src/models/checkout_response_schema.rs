use crate::models;
use serde::{Deserialize, Serialize};

/// CheckoutResponseSchema : Checkout session response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CheckoutResponseSchema {
    /// Stripe checkout URL for payment.
    #[serde(rename = "checkout_url")]
    pub checkout_url: String,
    /// Stripe checkout session ID.
    #[serde(rename = "session_id")]
    pub session_id: String,
}

impl CheckoutResponseSchema {
    /// Checkout session response.
    pub fn new(checkout_url: String, session_id: String) -> CheckoutResponseSchema {
        CheckoutResponseSchema {
            checkout_url,
            session_id,
        }
    }
}
