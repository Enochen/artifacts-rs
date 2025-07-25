use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct DepositWithdrawGoldSchema {
    /// Quantity of gold.
    #[serde(rename = "quantity")]
    pub quantity: u32,
}

impl DepositWithdrawGoldSchema {
    pub fn new(quantity: u32) -> DepositWithdrawGoldSchema {
        DepositWithdrawGoldSchema { quantity }
    }
}
