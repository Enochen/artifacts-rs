use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankSchema {
    /// Maximum slots in your bank.
    #[serde(rename = "slots")]
    pub slots: u32,
    /// Bank expansions.
    #[serde(rename = "expansions")]
    pub expansions: u32,
    /// Next expansion cost.
    #[serde(rename = "next_expansion_cost")]
    pub next_expansion_cost: u32,
    /// Quantity of gold in your bank.
    #[serde(rename = "gold")]
    pub gold: u32,
}

impl BankSchema {
    pub fn new(slots: u32, expansions: u32, next_expansion_cost: u32, gold: u32) -> BankSchema {
        BankSchema {
            slots,
            expansions,
            next_expansion_cost,
            gold,
        }
    }
}
