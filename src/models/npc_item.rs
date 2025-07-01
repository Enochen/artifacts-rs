use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NpcItem {
    /// The code of the NPC. This is the NPC's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// Code of the NPC that sells/buys the item.
    #[serde(rename = "npc")]
    pub npc: String,
    /// Currency used to buy/sell the item. If it's not gold, it's the item code.
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "buy_price", deserialize_with = "Option::deserialize")]
    pub buy_price: Option<i32>,
    #[serde(rename = "sell_price", deserialize_with = "Option::deserialize")]
    pub sell_price: Option<i32>,
}

impl NpcItem {
    pub fn new(
        code: String,
        npc: String,
        currency: String,
        buy_price: Option<i32>,
        sell_price: Option<i32>,
    ) -> NpcItem {
        NpcItem {
            code,
            npc,
            currency,
            buy_price,
            sell_price,
        }
    }
}
