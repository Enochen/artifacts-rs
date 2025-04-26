use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`accept_new_task`]
#[derive(Clone, Debug)]
pub struct AcceptNewTaskParams {
    /// Name of your character.
    pub name: String,
}

impl AcceptNewTaskParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`buy_bank_expansion`]
#[derive(Clone, Debug)]
pub struct BuyBankExpansionParams {
    /// Name of your character.
    pub name: String,
}

impl BuyBankExpansionParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`cancel_task`]
#[derive(Clone, Debug)]
pub struct CancelTaskParams {
    /// Name of your character.
    pub name: String,
}

impl CancelTaskParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`complete_task`]
#[derive(Clone, Debug)]
pub struct CompleteTaskParams {
    /// Name of your character.
    pub name: String,
}

impl CompleteTaskParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`craft`]
#[derive(Clone, Debug)]
pub struct CraftParams {
    /// Name of your character.
    pub name: String,
    pub crafting_schema: models::CraftingSchema,
}

impl CraftParams {
    pub fn new(name: String, crafting_schema: models::CraftingSchema) -> Self {
        Self {
            name,
            crafting_schema,
        }
    }
}

/// struct for passing parameters to the method [`delete_item`]
#[derive(Clone, Debug)]
pub struct DeleteItemParams {
    /// Name of your character.
    pub name: String,
    pub simple_item_schema: models::SimpleItemSchema,
}

impl DeleteItemParams {
    pub fn new(name: String, simple_item_schema: models::SimpleItemSchema) -> Self {
        Self {
            name,
            simple_item_schema,
        }
    }
}

/// struct for passing parameters to the method [`deposit_gold`]
#[derive(Clone, Debug)]
pub struct DepositGoldParams {
    /// Name of your character.
    pub name: String,
    pub deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
}

impl DepositGoldParams {
    pub fn new(
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
    ) -> Self {
        Self {
            name,
            deposit_withdraw_gold_schema,
        }
    }
}

/// struct for passing parameters to the method [`deposit_item`]
#[derive(Clone, Debug)]
pub struct DepositItemParams {
    /// Name of your character.
    pub name: String,
    pub simple_item_schema: models::SimpleItemSchema,
}

impl DepositItemParams {
    pub fn new(name: String, simple_item_schema: models::SimpleItemSchema) -> Self {
        Self {
            name,
            simple_item_schema,
        }
    }
}

/// struct for passing parameters to the method [`equip_item`]
#[derive(Clone, Debug)]
pub struct EquipItemParams {
    /// Name of your character.
    pub name: String,
    pub equip_schema: models::EquipSchema,
}

impl EquipItemParams {
    pub fn new(name: String, equip_schema: models::EquipSchema) -> Self {
        Self { name, equip_schema }
    }
}

/// struct for passing parameters to the method [`fight`]
#[derive(Clone, Debug)]
pub struct FightParams {
    /// Name of your character.
    pub name: String,
}

impl FightParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`gather`]
#[derive(Clone, Debug)]
pub struct GatherParams {
    /// Name of your character.
    pub name: String,
}

impl GatherParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`ge_buy_item`]
#[derive(Clone, Debug)]
pub struct GeBuyItemParams {
    /// Name of your character.
    pub name: String,
    pub ge_buy_order_schema: models::GeBuyOrderSchema,
}

impl GeBuyItemParams {
    pub fn new(name: String, ge_buy_order_schema: models::GeBuyOrderSchema) -> Self {
        Self {
            name,
            ge_buy_order_schema,
        }
    }
}

/// struct for passing parameters to the method [`ge_cancel_sell_order`]
#[derive(Clone, Debug)]
pub struct GeCancelSellOrderParams {
    /// Name of your character.
    pub name: String,
    pub ge_cancel_order_schema: models::GeCancelOrderSchema,
}

impl GeCancelSellOrderParams {
    pub fn new(name: String, ge_cancel_order_schema: models::GeCancelOrderSchema) -> Self {
        Self {
            name,
            ge_cancel_order_schema,
        }
    }
}

/// struct for passing parameters to the method [`ge_create_sell_order`]
#[derive(Clone, Debug)]
pub struct GeCreateSellOrderParams {
    /// Name of your character.
    pub name: String,
    pub ge_order_creationr_schema: models::GeOrderCreationrSchema,
}

impl GeCreateSellOrderParams {
    pub fn new(name: String, ge_order_creationr_schema: models::GeOrderCreationrSchema) -> Self {
        Self {
            name,
            ge_order_creationr_schema,
        }
    }
}

/// struct for passing parameters to the method [`get_all_characters_logs`]
#[derive(Clone, Debug)]
pub struct GetAllCharactersLogsParams {
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllCharactersLogsParams {
    pub fn new(page: Option<u32>, size: Option<u32>) -> Self {
        Self { page, size }
    }
}

/// struct for passing parameters to the method [`move_character`]
#[derive(Clone, Debug)]
pub struct MoveCharacterParams {
    /// Name of your character.
    pub name: String,
    pub destination_schema: models::DestinationSchema,
}

impl MoveCharacterParams {
    pub fn new(name: String, destination_schema: models::DestinationSchema) -> Self {
        Self {
            name,
            destination_schema,
        }
    }
}

/// struct for passing parameters to the method [`npc_buy_item`]
#[derive(Clone, Debug)]
pub struct NpcBuyItemParams {
    /// Name of your character.
    pub name: String,
    pub npc_merchant_buy_schema: models::NpcMerchantBuySchema,
}

impl NpcBuyItemParams {
    pub fn new(name: String, npc_merchant_buy_schema: models::NpcMerchantBuySchema) -> Self {
        Self {
            name,
            npc_merchant_buy_schema,
        }
    }
}

/// struct for passing parameters to the method [`npc_sell_item`]
#[derive(Clone, Debug)]
pub struct NpcSellItemParams {
    /// Name of your character.
    pub name: String,
    pub npc_merchant_buy_schema: models::NpcMerchantBuySchema,
}

impl NpcSellItemParams {
    pub fn new(name: String, npc_merchant_buy_schema: models::NpcMerchantBuySchema) -> Self {
        Self {
            name,
            npc_merchant_buy_schema,
        }
    }
}

/// struct for passing parameters to the method [`recycle`]
#[derive(Clone, Debug)]
pub struct RecycleParams {
    /// Name of your character.
    pub name: String,
    pub recycling_schema: models::RecyclingSchema,
}

impl RecycleParams {
    pub fn new(name: String, recycling_schema: models::RecyclingSchema) -> Self {
        Self {
            name,
            recycling_schema,
        }
    }
}

/// struct for passing parameters to the method [`rest_character`]
#[derive(Clone, Debug)]
pub struct RestCharacterParams {
    /// Name of your character.
    pub name: String,
}

impl RestCharacterParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`task_exchange`]
#[derive(Clone, Debug)]
pub struct TaskExchangeParams {
    /// Name of your character.
    pub name: String,
}

impl TaskExchangeParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for passing parameters to the method [`task_trade`]
#[derive(Clone, Debug)]
pub struct TaskTradeParams {
    /// Name of your character.
    pub name: String,
    pub simple_item_schema: models::SimpleItemSchema,
}

impl TaskTradeParams {
    pub fn new(name: String, simple_item_schema: models::SimpleItemSchema) -> Self {
        Self {
            name,
            simple_item_schema,
        }
    }
}

/// struct for passing parameters to the method [`unequip_item`]
#[derive(Clone, Debug)]
pub struct UnequipItemParams {
    /// Name of your character.
    pub name: String,
    pub unequip_schema: models::UnequipSchema,
}

impl UnequipItemParams {
    pub fn new(name: String, unequip_schema: models::UnequipSchema) -> Self {
        Self {
            name,
            unequip_schema,
        }
    }
}

/// struct for passing parameters to the method [`use_item`]
#[derive(Clone, Debug)]
pub struct UseItemParams {
    /// Name of your character.
    pub name: String,
    pub simple_item_schema: models::SimpleItemSchema,
}

impl UseItemParams {
    pub fn new(name: String, simple_item_schema: models::SimpleItemSchema) -> Self {
        Self {
            name,
            simple_item_schema,
        }
    }
}

/// struct for passing parameters to the method [`withdraw_gold`]
#[derive(Clone, Debug)]
pub struct WithdrawGoldParams {
    /// Name of your character.
    pub name: String,
    pub deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
}

impl WithdrawGoldParams {
    pub fn new(
        name: String,
        deposit_withdraw_gold_schema: models::DepositWithdrawGoldSchema,
    ) -> Self {
        Self {
            name,
            deposit_withdraw_gold_schema,
        }
    }
}

/// struct for passing parameters to the method [`withdraw_item`]
#[derive(Clone, Debug)]
pub struct WithdrawItemParams {
    /// Name of your character.
    pub name: String,
    pub simple_item_schema: models::SimpleItemSchema,
}

impl WithdrawItemParams {
    pub fn new(name: String, simple_item_schema: models::SimpleItemSchema) -> Self {
        Self {
            name,
            simple_item_schema,
        }
    }
}

/// struct for typed errors of method [`accept_new_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptNewTaskError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Tasks Master not found on this map.
    Status598,
    /// Character already has a task.
    Status489,
}

impl TryFrom<StatusCode> for AcceptNewTaskError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            598 => Ok(Self::Status598),
            489 => Ok(Self::Status489),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`buy_bank_expansion`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuyBankExpansionError {
    /// Bank not found on this map.
    Status598,
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Insufficient gold on your character.
    Status492,
}

impl TryFrom<StatusCode> for BuyBankExpansionError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            598 => Ok(Self::Status598),
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            492 => Ok(Self::Status492),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`cancel_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelTaskError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Tasks Master not found on this map.
    Status598,
    /// Missing item or insufficient quantity.
    Status478,
}

impl TryFrom<StatusCode> for CancelTaskError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            598 => Ok(Self::Status598),
            478 => Ok(Self::Status478),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`complete_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompleteTaskError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Tasks Master not found on this map.
    Status598,
    /// Character has not completed the task.
    Status488,
    /// Character has no task.
    Status487,
    /// Character inventory is full.
    Status497,
}

impl TryFrom<StatusCode> for CompleteTaskError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            598 => Ok(Self::Status598),
            488 => Ok(Self::Status488),
            487 => Ok(Self::Status487),
            497 => Ok(Self::Status497),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`craft`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CraftError {
    /// Craft not found.
    Status404,
    /// Workshop not found on this map.
    Status598,
    /// Character not found.
    Status498,
    /// Character inventory is full.
    Status497,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Not skill level required.
    Status493,
    /// Missing item or insufficient quantity.
    Status478,
}

impl TryFrom<StatusCode> for CraftError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            598 => Ok(Self::Status598),
            498 => Ok(Self::Status498),
            497 => Ok(Self::Status497),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            493 => Ok(Self::Status493),
            478 => Ok(Self::Status478),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`delete_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteItemError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Missing item or insufficient quantity.
    Status478,
}

impl TryFrom<StatusCode> for DeleteItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            478 => Ok(Self::Status478),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`deposit_gold`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DepositGoldError {
    /// Bank not found on this map.
    Status598,
    /// Insufficient gold on your character.
    Status492,
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// A transaction is already in progress with this item/your gold in your bank.
    Status461,
    /// An action is already in progress by your character.
    Status486,
}

impl TryFrom<StatusCode> for DepositGoldError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            598 => Ok(Self::Status598),
            492 => Ok(Self::Status492),
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            461 => Ok(Self::Status461),
            486 => Ok(Self::Status486),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`deposit_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DepositItemError {
    /// Bank not found on this map.
    Status598,
    /// Item not found.
    Status404,
    /// A transaction is already in progress with this item/your gold in your bank.
    Status461,
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Missing item or insufficient quantity.
    Status478,
    /// Your bank is full.
    Status462,
}

impl TryFrom<StatusCode> for DepositItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            598 => Ok(Self::Status598),
            404 => Ok(Self::Status404),
            461 => Ok(Self::Status461),
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            478 => Ok(Self::Status478),
            462 => Ok(Self::Status462),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`equip_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EquipItemError {
    /// Item not found.
    Status404,
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Missing item or insufficient quantity.
    Status478,
    /// Character level is insufficient.
    Status496,
    /// Slot is not empty.
    Status491,
    /// This item is already equipped.
    Status485,
    /// Character can&#39;t equip more than 100 utilities in the same slot.
    Status484,
    /// Character inventory is full.
    Status497,
}

impl TryFrom<StatusCode> for EquipItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            478 => Ok(Self::Status478),
            496 => Ok(Self::Status496),
            491 => Ok(Self::Status491),
            485 => Ok(Self::Status485),
            484 => Ok(Self::Status484),
            497 => Ok(Self::Status497),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`fight`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FightError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// Monster not found on this map.
    Status598,
    /// An action is already in progress by your character.
    Status486,
    /// Character inventory is full.
    Status497,
}

impl TryFrom<StatusCode> for FightError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            598 => Ok(Self::Status598),
            486 => Ok(Self::Status486),
            497 => Ok(Self::Status497),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`gather`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GatherError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// Resource not found on this map.
    Status598,
    /// An action is already in progress by your character.
    Status486,
    /// Not skill level required.
    Status493,
    /// Character inventory is full.
    Status497,
}

impl TryFrom<StatusCode> for GatherError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            598 => Ok(Self::Status598),
            486 => Ok(Self::Status486),
            493 => Ok(Self::Status493),
            497 => Ok(Self::Status497),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`ge_buy_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeBuyItemError {
    /// Grand Exchange not found on this map.
    Status598,
    /// Character not found.
    Status498,
    /// Character inventory is full.
    Status497,
    /// Character in cooldown.
    Status499,
    /// A transaction is already in progress on this order by a another character.
    Status436,
    /// An action is already in progress by your character.
    Status486,
    /// Insufficient gold on your character.
    Status492,
    /// This offer does not contain as many items.
    Status434,
    /// You can&#39;t buy to yourself.
    Status435,
    /// Order not found.
    Status404,
}

impl TryFrom<StatusCode> for GeBuyItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            598 => Ok(Self::Status598),
            498 => Ok(Self::Status498),
            497 => Ok(Self::Status497),
            499 => Ok(Self::Status499),
            436 => Ok(Self::Status436),
            486 => Ok(Self::Status486),
            492 => Ok(Self::Status492),
            434 => Ok(Self::Status434),
            435 => Ok(Self::Status435),
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`ge_cancel_sell_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeCancelSellOrderError {
    /// Grand Exchange not found on this map.
    Status598,
    /// Character not found.
    Status498,
    /// Character inventory is full.
    Status497,
    /// Character in cooldown.
    Status499,
    /// A transaction is already in progress on this order by a another character.
    Status436,
    /// An action is already in progress by your character.
    Status486,
    /// You can&#39;t cancel an order that is not yours.
    Status438,
    /// Order not found.
    Status404,
}

impl TryFrom<StatusCode> for GeCancelSellOrderError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            598 => Ok(Self::Status598),
            498 => Ok(Self::Status498),
            497 => Ok(Self::Status497),
            499 => Ok(Self::Status499),
            436 => Ok(Self::Status436),
            486 => Ok(Self::Status486),
            438 => Ok(Self::Status438),
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`ge_create_sell_order`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeCreateSellOrderError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Item not found.
    Status404,
    /// Missing item or insufficient quantity.
    Status478,
    /// Insufficient gold on your character.
    Status492,
    /// You can&#39;t create more than 100 orders at the same time.
    Status433,
    /// This item cannot be sold.
    Status437,
    /// Grand Exchange not found on this map.
    Status598,
}

impl TryFrom<StatusCode> for GeCreateSellOrderError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            404 => Ok(Self::Status404),
            478 => Ok(Self::Status478),
            492 => Ok(Self::Status492),
            433 => Ok(Self::Status433),
            437 => Ok(Self::Status437),
            598 => Ok(Self::Status598),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_all_characters_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllCharactersLogsError {
    /// Logs not found.
    Status404,
    /// Character not found.
    Status498,
}

impl TryFrom<StatusCode> for GetAllCharactersLogsError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            498 => Ok(Self::Status498),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_my_characters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyCharactersError {}

impl TryFrom<StatusCode> for GetMyCharactersError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`move_character`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MoveCharacterError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// Character already at destination.
    Status490,
    /// Map not found.
    Status404,
    /// An action is already in progress by your character.
    Status486,
}

impl TryFrom<StatusCode> for MoveCharacterError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            490 => Ok(Self::Status490),
            404 => Ok(Self::Status404),
            486 => Ok(Self::Status486),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`npc_buy_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NpcBuyItemError {
    /// NPC not found on this map.
    Status598,
    /// Character not found.
    Status498,
    /// Character inventory is full.
    Status497,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Insufficient gold on your character.
    Status492,
    /// This item cannot be purchased.
    Status441,
    /// Item not found.
    Status404,
}

impl TryFrom<StatusCode> for NpcBuyItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            598 => Ok(Self::Status598),
            498 => Ok(Self::Status498),
            497 => Ok(Self::Status497),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            492 => Ok(Self::Status492),
            441 => Ok(Self::Status441),
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`npc_sell_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NpcSellItemError {
    /// NPC not found on this map.
    Status598,
    /// Character not found.
    Status498,
    /// Character inventory is full.
    Status497,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Missing item or insufficient quantity.
    Status478,
    /// This item cannot be sold.
    Status442,
    /// Item not found.
    Status404,
}

impl TryFrom<StatusCode> for NpcSellItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            598 => Ok(Self::Status598),
            498 => Ok(Self::Status498),
            497 => Ok(Self::Status497),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            478 => Ok(Self::Status478),
            442 => Ok(Self::Status442),
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`recycle`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecycleError {
    /// Item not found.
    Status404,
    /// Workshop not found on this map.
    Status598,
    /// Character not found.
    Status498,
    /// Character inventory is full.
    Status497,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Not skill level required.
    Status493,
    /// Missing item or insufficient quantity.
    Status478,
    /// This item cannot be recycled.
    Status473,
}

impl TryFrom<StatusCode> for RecycleError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            598 => Ok(Self::Status598),
            498 => Ok(Self::Status498),
            497 => Ok(Self::Status497),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            493 => Ok(Self::Status493),
            478 => Ok(Self::Status478),
            473 => Ok(Self::Status473),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`rest_character`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RestCharacterError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
}

impl TryFrom<StatusCode> for RestCharacterError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`task_exchange`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskExchangeError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Tasks Master not found on this map.
    Status598,
    /// Missing item or insufficient quantity.
    Status478,
    /// Character inventory is full.
    Status497,
}

impl TryFrom<StatusCode> for TaskExchangeError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            598 => Ok(Self::Status598),
            478 => Ok(Self::Status478),
            497 => Ok(Self::Status497),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`task_trade`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskTradeError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// Tasks Master not found on this map.
    Status598,
    /// Character have already completed the task or are trying to trade too many items.
    Status475,
    /// Character does not have this task.
    Status474,
    /// Missing item or insufficient quantity.
    Status478,
}

impl TryFrom<StatusCode> for TaskTradeError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            598 => Ok(Self::Status598),
            475 => Ok(Self::Status475),
            474 => Ok(Self::Status474),
            478 => Ok(Self::Status478),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`unequip_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnequipItemError {
    /// Item not found.
    Status404,
    /// Character not found.
    Status498,
    /// An action is already in progress by your character.
    Status486,
    /// Slot is empty.
    Status491,
    /// Character inventory is full.
    Status497,
    /// Missing item or insufficient quantity.
    Status478,
    /// Character has no enough HP to unequip this item.
    Status483,
    /// Character in cooldown.
    Status499,
}

impl TryFrom<StatusCode> for UnequipItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            498 => Ok(Self::Status498),
            486 => Ok(Self::Status486),
            491 => Ok(Self::Status491),
            497 => Ok(Self::Status497),
            478 => Ok(Self::Status478),
            483 => Ok(Self::Status483),
            499 => Ok(Self::Status499),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`use_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UseItemError {
    /// Item not found.
    Status404,
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// An action is already in progress by your character.
    Status486,
    /// This item is not a consumable.
    Status476,
    /// Missing item or insufficient quantity.
    Status478,
    /// Character level is insufficient.
    Status496,
}

impl TryFrom<StatusCode> for UseItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            486 => Ok(Self::Status486),
            476 => Ok(Self::Status476),
            478 => Ok(Self::Status478),
            496 => Ok(Self::Status496),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`withdraw_gold`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WithdrawGoldError {
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// A transaction is already in progress with this item/your gold in your bank.
    Status461,
    /// An action is already in progress by your character.
    Status486,
    /// Bank not found on this map.
    Status598,
    /// Insufficient gold in your bank.
    Status460,
}

impl TryFrom<StatusCode> for WithdrawGoldError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            461 => Ok(Self::Status461),
            486 => Ok(Self::Status486),
            598 => Ok(Self::Status598),
            460 => Ok(Self::Status460),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`withdraw_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WithdrawItemError {
    /// Item not found.
    Status404,
    /// Character not found.
    Status498,
    /// Character in cooldown.
    Status499,
    /// A transaction is already in progress with this item/your gold in your bank.
    Status461,
    /// An action is already in progress by your character.
    Status486,
    /// Character inventory is full.
    Status497,
    /// Bank not found on this map.
    Status598,
    /// Missing item or insufficient quantity.
    Status478,
}

impl TryFrom<StatusCode> for WithdrawItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            498 => Ok(Self::Status498),
            499 => Ok(Self::Status499),
            461 => Ok(Self::Status461),
            486 => Ok(Self::Status486),
            497 => Ok(Self::Status497),
            598 => Ok(Self::Status598),
            478 => Ok(Self::Status478),
            _ => Err("status code not in spec"),
        }
    }
}

/// Accepting a new task.
pub async fn accept_new_task(
    configuration: &configuration::Configuration,
    params: AcceptNewTaskParams,
) -> Result<models::TaskResponseSchema, Error<AcceptNewTaskError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/task/new",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AcceptNewTaskError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Buy a 20 slots bank expansion.
pub async fn buy_bank_expansion(
    configuration: &configuration::Configuration,
    params: BuyBankExpansionParams,
) -> Result<models::BankExtensionTransactionResponseSchema, Error<BuyBankExpansionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/bank/buy_expansion",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BuyBankExpansionError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancel a task for 1 tasks coin.
pub async fn cancel_task(
    configuration: &configuration::Configuration,
    params: CancelTaskParams,
) -> Result<models::TaskCancelledResponseSchema, Error<CancelTaskError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/task/cancel",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CancelTaskError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Complete a task.
pub async fn complete_task(
    configuration: &configuration::Configuration,
    params: CompleteTaskParams,
) -> Result<models::RewardDataResponseSchema, Error<CompleteTaskError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/task/complete",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CompleteTaskError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Crafting an item. The character must be on a map with a workshop.
pub async fn craft(
    configuration: &configuration::Configuration,
    params: CraftParams,
) -> Result<models::SkillResponseSchema, Error<CraftError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let crafting_schema = params.crafting_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/crafting",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&crafting_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CraftError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an item from your character's inventory.
pub async fn delete_item(
    configuration: &configuration::Configuration,
    params: DeleteItemParams,
) -> Result<models::DeleteItemResponseSchema, Error<DeleteItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let simple_item_schema = params.simple_item_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/delete",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&simple_item_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deposit gold in a bank on the character's map.
pub async fn deposit_gold(
    configuration: &configuration::Configuration,
    params: DepositGoldParams,
) -> Result<models::BankGoldTransactionResponseSchema, Error<DepositGoldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let deposit_withdraw_gold_schema = params.deposit_withdraw_gold_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/bank/deposit/gold",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&deposit_withdraw_gold_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DepositGoldError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deposit an item in a bank on the character's map.
pub async fn deposit_item(
    configuration: &configuration::Configuration,
    params: DepositItemParams,
) -> Result<models::BankItemTransactionResponseSchema, Error<DepositItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let simple_item_schema = params.simple_item_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/bank/deposit",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&simple_item_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DepositItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Equip an item on your character.
pub async fn equip_item(
    configuration: &configuration::Configuration,
    params: EquipItemParams,
) -> Result<models::EquipmentResponseSchema, Error<EquipItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let equip_schema = params.equip_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/equip",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&equip_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EquipItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Start a fight against a monster on the character's map.
pub async fn fight(
    configuration: &configuration::Configuration,
    params: FightParams,
) -> Result<models::CharacterFightResponseSchema, Error<FightError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/fight",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FightError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Harvest a resource on the character's map.
pub async fn gather(
    configuration: &configuration::Configuration,
    params: GatherParams,
) -> Result<models::SkillResponseSchema, Error<GatherError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/gathering",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GatherError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Buy an item at the Grand Exchange on the character's map.
pub async fn ge_buy_item(
    configuration: &configuration::Configuration,
    params: GeBuyItemParams,
) -> Result<models::GeTransactionResponseSchema, Error<GeBuyItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let ge_buy_order_schema = params.ge_buy_order_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/grandexchange/buy",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&ge_buy_order_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GeBuyItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancel a sell order at the Grand Exchange on the character's map.
pub async fn ge_cancel_sell_order(
    configuration: &configuration::Configuration,
    params: GeCancelSellOrderParams,
) -> Result<models::GeTransactionResponseSchema, Error<GeCancelSellOrderError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let ge_cancel_order_schema = params.ge_cancel_order_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/grandexchange/cancel",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&ge_cancel_order_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GeCancelSellOrderError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a sell order at the Grand Exchange on the character's map. Please note there is a 3% listing tax, charged at the time of posting, on the total price.
pub async fn ge_create_sell_order(
    configuration: &configuration::Configuration,
    params: GeCreateSellOrderParams,
) -> Result<models::GeCreateOrderTransactionResponseSchema, Error<GeCreateSellOrderError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let ge_order_creationr_schema = params.ge_order_creationr_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/grandexchange/sell",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&ge_order_creationr_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GeCreateSellOrderError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// History of the last 100 actions of all your characters.
pub async fn get_all_characters_logs(
    configuration: &configuration::Configuration,
    params: GetAllCharactersLogsParams,
) -> Result<models::DataPageLogSchema, Error<GetAllCharactersLogsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/my/logs", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = size {
        local_var_req_builder =
            local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllCharactersLogsError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List of your characters. This endpoint is deprecated and will be removed in a future version. Please use accounts/{account}/characters.
pub async fn get_my_characters(
    configuration: &configuration::Configuration,
) -> Result<models::MyCharactersListSchema, Error<GetMyCharactersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/my/characters", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMyCharactersError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Moves a character on the map using the map's X and Y position.
pub async fn move_character(
    configuration: &configuration::Configuration,
    params: MoveCharacterParams,
) -> Result<models::CharacterMovementResponseSchema, Error<MoveCharacterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let destination_schema = params.destination_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/move",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&destination_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MoveCharacterError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Buy an item from an NPC on the character's map.
pub async fn npc_buy_item(
    configuration: &configuration::Configuration,
    params: NpcBuyItemParams,
) -> Result<models::NpcMerchantTransactionResponseSchema, Error<NpcBuyItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let npc_merchant_buy_schema = params.npc_merchant_buy_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/npc/buy",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&npc_merchant_buy_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NpcBuyItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sell an item to an NPC on the character's map.
pub async fn npc_sell_item(
    configuration: &configuration::Configuration,
    params: NpcSellItemParams,
) -> Result<models::NpcMerchantTransactionResponseSchema, Error<NpcSellItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let npc_merchant_buy_schema = params.npc_merchant_buy_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/npc/sell",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&npc_merchant_buy_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<NpcSellItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Recycling an item. The character must be on a map with a workshop (only for equipments and weapons).
pub async fn recycle(
    configuration: &configuration::Configuration,
    params: RecycleParams,
) -> Result<models::RecyclingResponseSchema, Error<RecycleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let recycling_schema = params.recycling_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/recycling",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&recycling_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RecycleError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Recovers hit points by resting. (1 second per 5 HP, minimum 3 seconds)
pub async fn rest_character(
    configuration: &configuration::Configuration,
    params: RestCharacterParams,
) -> Result<models::CharacterRestResponseSchema, Error<RestCharacterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/rest",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RestCharacterError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Exchange 6 tasks coins for a random reward. Rewards are exclusive items or resources.
pub async fn task_exchange(
    configuration: &configuration::Configuration,
    params: TaskExchangeParams,
) -> Result<models::RewardDataResponseSchema, Error<TaskExchangeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/task/exchange",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TaskExchangeError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Trading items with a Tasks Master.
pub async fn task_trade(
    configuration: &configuration::Configuration,
    params: TaskTradeParams,
) -> Result<models::TaskTradeResponseSchema, Error<TaskTradeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let simple_item_schema = params.simple_item_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/task/trade",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&simple_item_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TaskTradeError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unequip an item on your character.
pub async fn unequip_item(
    configuration: &configuration::Configuration,
    params: UnequipItemParams,
) -> Result<models::EquipmentResponseSchema, Error<UnequipItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let unequip_schema = params.unequip_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/unequip",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&unequip_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UnequipItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Use an item as a consumable.
pub async fn use_item(
    configuration: &configuration::Configuration,
    params: UseItemParams,
) -> Result<models::UseItemResponseSchema, Error<UseItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let simple_item_schema = params.simple_item_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/use",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&simple_item_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UseItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Withdraw gold from your bank.
pub async fn withdraw_gold(
    configuration: &configuration::Configuration,
    params: WithdrawGoldParams,
) -> Result<models::BankGoldTransactionResponseSchema, Error<WithdrawGoldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let deposit_withdraw_gold_schema = params.deposit_withdraw_gold_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/bank/withdraw/gold",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&deposit_withdraw_gold_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WithdrawGoldError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Take an item from your bank and put it in the character's inventory.
pub async fn withdraw_item(
    configuration: &configuration::Configuration,
    params: WithdrawItemParams,
) -> Result<models::BankItemTransactionResponseSchema, Error<WithdrawItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let simple_item_schema = params.simple_item_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/bank/withdraw",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&simple_item_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WithdrawItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
