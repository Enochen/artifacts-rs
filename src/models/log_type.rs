use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum LogType {
    #[serde(rename = "spawn")]
    Spawn,
    #[serde(rename = "movement")]
    Movement,
    #[serde(rename = "fight")]
    Fight,
    #[serde(rename = "crafting")]
    Crafting,
    #[serde(rename = "gathering")]
    Gathering,
    #[serde(rename = "buy_ge")]
    BuyGe,
    #[serde(rename = "sell_ge")]
    SellGe,
    #[serde(rename = "buy_npc")]
    BuyNpc,
    #[serde(rename = "sell_npc")]
    SellNpc,
    #[serde(rename = "cancel_ge")]
    CancelGe,
    #[serde(rename = "delete_item")]
    DeleteItem,
    #[serde(rename = "deposit_item")]
    DepositItem,
    #[serde(rename = "withdraw_item")]
    WithdrawItem,
    #[serde(rename = "deposit_gold")]
    DepositGold,
    #[serde(rename = "withdraw_gold")]
    WithdrawGold,
    #[serde(rename = "equip")]
    Equip,
    #[serde(rename = "unequip")]
    Unequip,
    #[serde(rename = "new_task")]
    NewTask,
    #[serde(rename = "task_exchange")]
    TaskExchange,
    #[serde(rename = "task_cancelled")]
    TaskCancelled,
    #[serde(rename = "task_completed")]
    TaskCompleted,
    #[serde(rename = "task_trade")]
    TaskTrade,
    #[serde(rename = "christmas_exchange")]
    ChristmasExchange,
    #[serde(rename = "recycling")]
    Recycling,
    #[serde(rename = "rest")]
    Rest,
    #[serde(rename = "use")]
    Use,
    #[serde(rename = "buy_bank_expansion")]
    BuyBankExpansion,
    #[serde(rename = "achievement")]
    Achievement,
    #[serde(rename = "give_item")]
    GiveItem,
    #[serde(rename = "give_gold")]
    GiveGold,
    #[serde(rename = "receive_item")]
    ReceiveItem,
    #[serde(rename = "receive_gold")]
    ReceiveGold,
    #[serde(rename = "change_skin")]
    ChangeSkin,
    #[serde(rename = "rename")]
    Rename,
}

impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Spawn => write!(f, "spawn"),
            Self::Movement => write!(f, "movement"),
            Self::Fight => write!(f, "fight"),
            Self::Crafting => write!(f, "crafting"),
            Self::Gathering => write!(f, "gathering"),
            Self::BuyGe => write!(f, "buy_ge"),
            Self::SellGe => write!(f, "sell_ge"),
            Self::BuyNpc => write!(f, "buy_npc"),
            Self::SellNpc => write!(f, "sell_npc"),
            Self::CancelGe => write!(f, "cancel_ge"),
            Self::DeleteItem => write!(f, "delete_item"),
            Self::DepositItem => write!(f, "deposit_item"),
            Self::WithdrawItem => write!(f, "withdraw_item"),
            Self::DepositGold => write!(f, "deposit_gold"),
            Self::WithdrawGold => write!(f, "withdraw_gold"),
            Self::Equip => write!(f, "equip"),
            Self::Unequip => write!(f, "unequip"),
            Self::NewTask => write!(f, "new_task"),
            Self::TaskExchange => write!(f, "task_exchange"),
            Self::TaskCancelled => write!(f, "task_cancelled"),
            Self::TaskCompleted => write!(f, "task_completed"),
            Self::TaskTrade => write!(f, "task_trade"),
            Self::ChristmasExchange => write!(f, "christmas_exchange"),
            Self::Recycling => write!(f, "recycling"),
            Self::Rest => write!(f, "rest"),
            Self::Use => write!(f, "use"),
            Self::BuyBankExpansion => write!(f, "buy_bank_expansion"),
            Self::Achievement => write!(f, "achievement"),
            Self::GiveItem => write!(f, "give_item"),
            Self::GiveGold => write!(f, "give_gold"),
            Self::ReceiveItem => write!(f, "receive_item"),
            Self::ReceiveGold => write!(f, "receive_gold"),
            Self::ChangeSkin => write!(f, "change_skin"),
            Self::Rename => write!(f, "rename"),
        }
    }
}

impl Default for LogType {
    fn default() -> LogType {
        Self::Spawn
    }
}
