use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct SimpleNpcItemSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Currency used to buy/sell the item. If it's not gold, it's the item code.
    #[serde(rename = "currency")]
    pub currency: String,
    /// Price to buy the item.
    #[serde(rename = "buy_price", skip_serializing_if = "Option::is_none")]
    pub buy_price: Option<i32>,
    /// Price to sell the item.
    #[serde(rename = "sell_price", skip_serializing_if = "Option::is_none")]
    pub sell_price: Option<i32>,
}

impl SimpleNpcItemSchema {
    pub fn new(code: String, currency: String) -> SimpleNpcItemSchema {
        SimpleNpcItemSchema {
            code,
            currency,
            buy_price: None,
            sell_price: None,
        }
    }
}
