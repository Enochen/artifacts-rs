use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct PurchaseHistorySchema {
    /// Type of purchase.
    #[serde(rename = "type")]
    pub r#type: models::PurchaseType,
    /// Description of the purchase.
    #[serde(rename = "description")]
    pub description: String,
    /// Amount in cents.
    #[serde(rename = "amount")]
    pub amount: i32,
    /// Gems credited from this purchase.
    #[serde(rename = "gems_credited", skip_serializing_if = "Option::is_none")]
    pub gems_credited: Option<i32>,
    /// When the purchase was made.
    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl PurchaseHistorySchema {
    pub fn new(
        r#type: models::PurchaseType,
        description: String,
        amount: i32,
        created_at: String,
    ) -> PurchaseHistorySchema {
        PurchaseHistorySchema {
            r#type,
            description,
            amount,
            gems_credited: None,
            created_at,
        }
    }
}
