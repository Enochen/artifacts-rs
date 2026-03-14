use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

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

/// struct for passing parameters to the method [`action_transition`]
#[derive(Clone, Debug)]
pub struct ActionTransitionParams {
    /// Name of your character.
    pub name: String,
}

impl ActionTransitionParams {
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

/// struct for passing parameters to the method [`change_skin`]
#[derive(Clone, Debug)]
pub struct ChangeSkinParams {
    /// Name of your character.
    pub name: String,
    pub change_skin_character_schema: models::ChangeSkinCharacterSchema,
}

impl ChangeSkinParams {
    pub fn new(
        name: String,
        change_skin_character_schema: models::ChangeSkinCharacterSchema,
    ) -> Self {
        Self {
            name,
            change_skin_character_schema,
        }
    }
}

/// struct for passing parameters to the method [`claim_pending_item`]
#[derive(Clone, Debug)]
pub struct ClaimPendingItemParams {
    /// Name of your character.
    pub name: String,
    /// The ID of the pending item to claim.
    pub id: String,
}

impl ClaimPendingItemParams {
    pub fn new(name: String, id: String) -> Self {
        Self { name, id }
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
    pub simple_item_schema: Vec<models::SimpleItemSchema>,
}

impl DepositItemParams {
    pub fn new(name: String, simple_item_schema: Vec<models::SimpleItemSchema>) -> Self {
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
    pub fight_request_schema: Option<models::FightRequestSchema>,
}

impl FightParams {
    pub fn new(name: String, fight_request_schema: Option<models::FightRequestSchema>) -> Self {
        Self {
            name,
            fight_request_schema,
        }
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

/// struct for passing parameters to the method [`ge_cancel_order`]
#[derive(Clone, Debug)]
pub struct GeCancelOrderParams {
    /// Name of your character.
    pub name: String,
    pub ge_cancel_order_schema: models::GeCancelOrderSchema,
}

impl GeCancelOrderParams {
    pub fn new(name: String, ge_cancel_order_schema: models::GeCancelOrderSchema) -> Self {
        Self {
            name,
            ge_cancel_order_schema,
        }
    }
}

/// struct for passing parameters to the method [`ge_create_buy_order`]
#[derive(Clone, Debug)]
pub struct GeCreateBuyOrderParams {
    /// Name of your character.
    pub name: String,
    pub ge_buy_order_creation_schema: models::GeBuyOrderCreationSchema,
}

impl GeCreateBuyOrderParams {
    pub fn new(
        name: String,
        ge_buy_order_creation_schema: models::GeBuyOrderCreationSchema,
    ) -> Self {
        Self {
            name,
            ge_buy_order_creation_schema,
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

/// struct for passing parameters to the method [`ge_fill_order`]
#[derive(Clone, Debug)]
pub struct GeFillOrderParams {
    /// Name of your character.
    pub name: String,
    pub ge_fill_buy_order_schema: models::GeFillBuyOrderSchema,
}

impl GeFillOrderParams {
    pub fn new(name: String, ge_fill_buy_order_schema: models::GeFillBuyOrderSchema) -> Self {
        Self {
            name,
            ge_fill_buy_order_schema,
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

/// struct for passing parameters to the method [`get_character_logs`]
#[derive(Clone, Debug)]
pub struct GetCharacterLogsParams {
    /// Name of your character.
    pub name: String,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetCharacterLogsParams {
    pub fn new(name: String, page: Option<u32>, size: Option<u32>) -> Self {
        Self { name, page, size }
    }
}

/// struct for passing parameters to the method [`give_gold`]
#[derive(Clone, Debug)]
pub struct GiveGoldParams {
    /// Name of your character.
    pub name: String,
    pub give_gold_schema: models::GiveGoldSchema,
}

impl GiveGoldParams {
    pub fn new(name: String, give_gold_schema: models::GiveGoldSchema) -> Self {
        Self {
            name,
            give_gold_schema,
        }
    }
}

/// struct for passing parameters to the method [`give_items`]
#[derive(Clone, Debug)]
pub struct GiveItemsParams {
    /// Name of your character.
    pub name: String,
    pub give_items_schema: models::GiveItemsSchema,
}

impl GiveItemsParams {
    pub fn new(name: String, give_items_schema: models::GiveItemsSchema) -> Self {
        Self {
            name,
            give_items_schema,
        }
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
    pub simple_item_schema: Vec<models::SimpleItemSchema>,
}

impl WithdrawItemParams {
    pub fn new(name: String, simple_item_schema: Vec<models::SimpleItemSchema>) -> Self {
        Self {
            name,
            simple_item_schema,
        }
    }
}

/// struct for typed errors of method [`accept_new_task`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AcceptNewTaskError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Tasks Master not found on this map.
    Status598(models::ErrorResponseSchema),
    /// The character already has an assigned task.
    Status489(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for AcceptNewTaskError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            598 => Ok(Self::Status598(raw)),
            489 => Ok(Self::Status489(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`action_transition`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ActionTransitionError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// Transition not found.
    Status404(models::ErrorResponseSchema),
    /// Insufficient gold for this transition.
    Status492(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Conditions not met.
    Status496(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for ActionTransitionError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            404 => Ok(Self::Status404(raw)),
            492 => Ok(Self::Status492(raw)),
            478 => Ok(Self::Status478(raw)),
            486 => Ok(Self::Status486(raw)),
            496 => Ok(Self::Status496(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`buy_bank_expansion`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BuyBankExpansionError {
    /// Bank not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character does not have enough gold.
    Status492(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for BuyBankExpansionError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            492 => Ok(Self::Status492(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`cancel_task`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CancelTaskError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character has no task assigned.
    Status487(models::ErrorResponseSchema),
    /// Tasks Master not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for CancelTaskError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            487 => Ok(Self::Status487(raw)),
            598 => Ok(Self::Status598(raw)),
            478 => Ok(Self::Status478(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`change_skin`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChangeSkinError {
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// You cannot choose this skin because you do not own it.
    Status550(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for ChangeSkinError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            550 => Ok(Self::Status550(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`claim_pending_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ClaimPendingItemError {
    /// Pending item not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for ClaimPendingItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            497 => Ok(Self::Status497(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`complete_task`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CompleteTaskError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Tasks Master not found on this map.
    Status598(models::ErrorResponseSchema),
    /// The character has not completed the task.
    Status488(models::ErrorResponseSchema),
    /// The character has no task assigned.
    Status487(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for CompleteTaskError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            598 => Ok(Self::Status598(raw)),
            488 => Ok(Self::Status488(raw)),
            487 => Ok(Self::Status487(raw)),
            497 => Ok(Self::Status497(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`craft`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CraftError {
    /// Craft not found.
    Status404(models::ErrorResponseSchema),
    /// Workshop not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character&#39;s skill level is too low.
    Status493(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for CraftError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            497 => Ok(Self::Status497(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            493 => Ok(Self::Status493(raw)),
            478 => Ok(Self::Status478(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`delete_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DeleteItemError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for DeleteItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            478 => Ok(Self::Status478(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`deposit_gold`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DepositGoldError {
    /// Bank not found on this map.
    Status598(models::ErrorResponseSchema),
    /// The character does not have enough gold.
    Status492(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// Some of your items or your gold in the bank are already part of an ongoing transaction.
    Status461(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for DepositGoldError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            492 => Ok(Self::Status492(raw)),
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            461 => Ok(Self::Status461(raw)),
            486 => Ok(Self::Status486(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`deposit_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DepositItemError {
    /// Bank not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Some of your items or your gold in the bank are already part of an ongoing transaction.
    Status461(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Your bank is full.
    Status462(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for DepositItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            404 => Ok(Self::Status404(raw)),
            461 => Ok(Self::Status461(raw)),
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            478 => Ok(Self::Status478(raw)),
            462 => Ok(Self::Status462(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`equip_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum EquipItemError {
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character does not have enough HP to unequip this item.
    Status483(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Conditions not met.
    Status496(models::ErrorResponseSchema),
    /// The equipment slot is not empty.
    Status491(models::ErrorResponseSchema),
    /// This item is already equipped.
    Status485(models::ErrorResponseSchema),
    /// The character cannot equip more than 100 utilities in the same slot.
    Status484(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for EquipItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            483 => Ok(Self::Status483(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            478 => Ok(Self::Status478(raw)),
            496 => Ok(Self::Status496(raw)),
            491 => Ok(Self::Status491(raw)),
            485 => Ok(Self::Status485(raw)),
            484 => Ok(Self::Status484(raw)),
            497 => Ok(Self::Status497(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`fight`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FightError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// Monster not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Only boss monsters can be fought by multiple characters.
    Status486(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for FightError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            598 => Ok(Self::Status598(raw)),
            486 => Ok(Self::Status486(raw)),
            497 => Ok(Self::Status497(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`gather`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GatherError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// Resource not found on this map.
    Status598(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character&#39;s skill level is too low.
    Status493(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GatherError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            598 => Ok(Self::Status598(raw)),
            486 => Ok(Self::Status486(raw)),
            493 => Ok(Self::Status493(raw)),
            497 => Ok(Self::Status497(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`ge_buy_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GeBuyItemError {
    /// Grand Exchange not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// A transaction is already in progress for this order by another character.
    Status436(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character does not have enough gold.
    Status492(models::ErrorResponseSchema),
    /// This offer does not contain that many items.
    Status434(models::ErrorResponseSchema),
    /// You cannot trade with yourself.
    Status435(models::ErrorResponseSchema),
    /// Order not found.
    Status404(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GeBuyItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            497 => Ok(Self::Status497(raw)),
            499 => Ok(Self::Status499(raw)),
            436 => Ok(Self::Status436(raw)),
            486 => Ok(Self::Status486(raw)),
            492 => Ok(Self::Status492(raw)),
            434 => Ok(Self::Status434(raw)),
            435 => Ok(Self::Status435(raw)),
            404 => Ok(Self::Status404(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`ge_cancel_order`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GeCancelOrderError {
    /// Grand Exchange not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// A transaction is already in progress for this order by another character.
    Status436(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// You cannot cancel an order that is not yours.
    Status438(models::ErrorResponseSchema),
    /// Order not found.
    Status404(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GeCancelOrderError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            497 => Ok(Self::Status497(raw)),
            499 => Ok(Self::Status499(raw)),
            436 => Ok(Self::Status436(raw)),
            486 => Ok(Self::Status486(raw)),
            438 => Ok(Self::Status438(raw)),
            404 => Ok(Self::Status404(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`ge_create_buy_order`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GeCreateBuyOrderError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// The character does not have enough gold.
    Status492(models::ErrorResponseSchema),
    /// You cannot create more than 100 orders at the same time.
    Status433(models::ErrorResponseSchema),
    /// This item cannot be sold.
    Status437(models::ErrorResponseSchema),
    /// Grand Exchange not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GeCreateBuyOrderError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            404 => Ok(Self::Status404(raw)),
            492 => Ok(Self::Status492(raw)),
            433 => Ok(Self::Status433(raw)),
            437 => Ok(Self::Status437(raw)),
            598 => Ok(Self::Status598(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`ge_create_sell_order`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GeCreateSellOrderError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// You cannot create more than 100 orders at the same time.
    Status433(models::ErrorResponseSchema),
    /// This item cannot be sold.
    Status437(models::ErrorResponseSchema),
    /// Grand Exchange not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GeCreateSellOrderError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            404 => Ok(Self::Status404(raw)),
            478 => Ok(Self::Status478(raw)),
            433 => Ok(Self::Status433(raw)),
            437 => Ok(Self::Status437(raw)),
            598 => Ok(Self::Status598(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`ge_fill_order`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GeFillOrderError {
    /// Grand Exchange not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// A transaction is already in progress for this order by another character.
    Status436(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// This offer does not contain that many items.
    Status434(models::ErrorResponseSchema),
    /// You cannot trade with yourself.
    Status435(models::ErrorResponseSchema),
    /// Buy order not found.
    Status404(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GeFillOrderError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            436 => Ok(Self::Status436(raw)),
            486 => Ok(Self::Status486(raw)),
            478 => Ok(Self::Status478(raw)),
            434 => Ok(Self::Status434(raw)),
            435 => Ok(Self::Status435(raw)),
            404 => Ok(Self::Status404(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`get_all_characters_logs`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllCharactersLogsError {
    /// Logs not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetAllCharactersLogsError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`get_character_logs`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetCharacterLogsError {
    /// Logs not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetCharacterLogsError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`get_my_characters`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetMyCharactersError {}

impl<'de> Deserialize<'de> for GetMyCharactersError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        Err(de::Error::custom(format!(
            "Unexpected error code: {}",
            raw.error.code
        )))
    }
}

/// struct for typed errors of method [`give_gold`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiveGoldError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// The character does not have enough gold.
    Status492(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GiveGoldError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            492 => Ok(Self::Status492(raw)),
            486 => Ok(Self::Status486(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`give_items`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiveItemsError {
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GiveItemsError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            497 => Ok(Self::Status497(raw)),
            486 => Ok(Self::Status486(raw)),
            478 => Ok(Self::Status478(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`move_character`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MoveCharacterError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// The character is already at the destination.
    Status490(models::ErrorResponseSchema),
    /// Map not found.
    Status404(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// No path available to the destination map.
    Status595(models::ErrorResponseSchema),
    /// The map is blocked and cannot be accessed.
    Status596(models::ErrorResponseSchema),
    /// Conditions not met.
    Status496(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for MoveCharacterError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            490 => Ok(Self::Status490(raw)),
            404 => Ok(Self::Status404(raw)),
            486 => Ok(Self::Status486(raw)),
            595 => Ok(Self::Status595(raw)),
            596 => Ok(Self::Status596(raw)),
            496 => Ok(Self::Status496(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`npc_buy_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NpcBuyItemError {
    /// NPC not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character does not have enough gold.
    Status492(models::ErrorResponseSchema),
    /// This item is not available for purchase.
    Status441(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for NpcBuyItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            497 => Ok(Self::Status497(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            492 => Ok(Self::Status492(raw)),
            441 => Ok(Self::Status441(raw)),
            478 => Ok(Self::Status478(raw)),
            404 => Ok(Self::Status404(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`npc_sell_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NpcSellItemError {
    /// NPC not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// This item cannot be sold.
    Status442(models::ErrorResponseSchema),
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for NpcSellItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            497 => Ok(Self::Status497(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            478 => Ok(Self::Status478(raw)),
            442 => Ok(Self::Status442(raw)),
            404 => Ok(Self::Status404(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`recycle`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RecycleError {
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Workshop not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character&#39;s skill level is too low.
    Status493(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// This item cannot be recycled.
    Status473(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for RecycleError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            598 => Ok(Self::Status598(raw)),
            498 => Ok(Self::Status498(raw)),
            497 => Ok(Self::Status497(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            493 => Ok(Self::Status493(raw)),
            478 => Ok(Self::Status478(raw)),
            473 => Ok(Self::Status473(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`rest_character`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RestCharacterError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for RestCharacterError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`task_exchange`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TaskExchangeError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Tasks Master not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for TaskExchangeError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            598 => Ok(Self::Status598(raw)),
            478 => Ok(Self::Status478(raw)),
            497 => Ok(Self::Status497(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`task_trade`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TaskTradeError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Tasks Master not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Task already completed or too many items submitted.
    Status475(models::ErrorResponseSchema),
    /// The character does not have this task.
    Status474(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for TaskTradeError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            598 => Ok(Self::Status598(raw)),
            475 => Ok(Self::Status475(raw)),
            474 => Ok(Self::Status474(raw)),
            478 => Ok(Self::Status478(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`unequip_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UnequipItemError {
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The equipment slot is empty.
    Status491(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// The character does not have enough HP to unequip this item.
    Status483(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for UnequipItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            486 => Ok(Self::Status486(raw)),
            491 => Ok(Self::Status491(raw)),
            497 => Ok(Self::Status497(raw)),
            478 => Ok(Self::Status478(raw)),
            483 => Ok(Self::Status483(raw)),
            499 => Ok(Self::Status499(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`use_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UseItemError {
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// This item is not a consumable.
    Status476(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Conditions not met.
    Status496(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for UseItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            486 => Ok(Self::Status486(raw)),
            476 => Ok(Self::Status476(raw)),
            478 => Ok(Self::Status478(raw)),
            496 => Ok(Self::Status496(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`withdraw_gold`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WithdrawGoldError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// Some of your items or your gold in the bank are already part of an ongoing transaction.
    Status461(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// Bank not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Insufficient gold in your bank.
    Status460(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for WithdrawGoldError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            461 => Ok(Self::Status461(raw)),
            486 => Ok(Self::Status486(raw)),
            598 => Ok(Self::Status598(raw)),
            460 => Ok(Self::Status460(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`withdraw_item`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum WithdrawItemError {
    /// Item not found.
    Status404(models::ErrorResponseSchema),
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// The character is in cooldown.
    Status499(models::ErrorResponseSchema),
    /// Some of your items or your gold in the bank are already part of an ongoing transaction.
    Status461(models::ErrorResponseSchema),
    /// An action is already in progress for this character.
    Status486(models::ErrorResponseSchema),
    /// The character&#39;s inventory is full.
    Status497(models::ErrorResponseSchema),
    /// Bank not found on this map.
    Status598(models::ErrorResponseSchema),
    /// Missing required item(s).
    Status478(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for WithdrawItemError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            498 => Ok(Self::Status498(raw)),
            499 => Ok(Self::Status499(raw)),
            461 => Ok(Self::Status461(raw)),
            486 => Ok(Self::Status486(raw)),
            497 => Ok(Self::Status497(raw)),
            598 => Ok(Self::Status598(raw)),
            478 => Ok(Self::Status478(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
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
        let local_var_entity: Option<AcceptNewTaskError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Execute a transition from the current map to another layer. The character must be on a map that has a transition available.
pub async fn action_transition(
    configuration: &configuration::Configuration,
    params: ActionTransitionParams,
) -> Result<models::CharacterTransitionResponseSchema, Error<ActionTransitionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/transition",
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
        let local_var_entity: Option<ActionTransitionError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<BuyBankExpansionError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<CancelTaskError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Change the skin of your character.
pub async fn change_skin(
    configuration: &configuration::Configuration,
    params: ChangeSkinParams,
) -> Result<models::ChangeSkinResponseSchema, Error<ChangeSkinError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let change_skin_character_schema = params.change_skin_character_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/change_skin",
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
    local_var_req_builder = local_var_req_builder.json(&change_skin_character_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChangeSkinError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Claim a pending item with a specific character.
pub async fn claim_pending_item(
    configuration: &configuration::Configuration,
    params: ClaimPendingItemParams,
) -> Result<models::ClaimPendingItemResponseSchema, Error<ClaimPendingItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/claim_item/{id}",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name),
        id = crate::apis::urlencode(id)
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
        let local_var_entity: Option<ClaimPendingItemError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<CompleteTaskError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Craft an item. The character must be on a map with a workshop.
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
        let local_var_entity: Option<CraftError> = serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<DeleteItemError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<DepositGoldError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deposit multiple items in a bank on the character's map. The cooldown will be 3 seconds multiplied by the number of different items deposited.
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
        "{}/my/{name}/action/bank/deposit/item",
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
        let local_var_entity: Option<DepositItemError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<EquipItemError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Start a fight against a monster on the character's map. Add participants for multi-character fights (up to 3 characters, only for boss).
pub async fn fight(
    configuration: &configuration::Configuration,
    params: FightParams,
) -> Result<models::CharacterFightResponseSchema, Error<FightError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let fight_request_schema = params.fight_request_schema;

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
    local_var_req_builder = local_var_req_builder.json(&fight_request_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FightError> = serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<GatherError> = serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<GeBuyItemError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancel an order (sell or buy) at the Grand Exchange on the character's map.  For sell orders: Items are returned to your inventory. For buy orders: Gold is refunded to your character.
pub async fn ge_cancel_order(
    configuration: &configuration::Configuration,
    params: GeCancelOrderParams,
) -> Result<models::GeTransactionResponseSchema, Error<GeCancelOrderError>> {
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
        let local_var_entity: Option<GeCancelOrderError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a buy order at the Grand Exchange on the character's map.  The total gold (price * quantity) is locked when creating the order. Other players can then sell items to fulfill your order. Items will be delivered to your pending items when the order is filled.
pub async fn ge_create_buy_order(
    configuration: &configuration::Configuration,
    params: GeCreateBuyOrderParams,
) -> Result<models::GeCreateOrderTransactionResponseSchema, Error<GeCreateBuyOrderError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let ge_buy_order_creation_schema = params.ge_buy_order_creation_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/grandexchange/create-buy-order",
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
    local_var_req_builder = local_var_req_builder.json(&ge_buy_order_creation_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GeCreateBuyOrderError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a sell order at the Grand Exchange on the character's map.
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
        "{}/my/{name}/action/grandexchange/create-sell-order",
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
        let local_var_entity: Option<GeCreateSellOrderError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sell items to an existing buy order at the Grand Exchange on the character's map.  You will receive the gold immediately. The buyer will receive the items in their pending items.
pub async fn ge_fill_order(
    configuration: &configuration::Configuration,
    params: GeFillOrderParams,
) -> Result<models::GeTransactionResponseSchema, Error<GeFillOrderError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let ge_fill_buy_order_schema = params.ge_fill_buy_order_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/grandexchange/fill",
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
    local_var_req_builder = local_var_req_builder.json(&ge_fill_buy_order_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GeFillOrderError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// History of the last 5000 actions of all your characters.
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
        let local_var_entity: Option<GetAllCharactersLogsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// History of the last actions of your character.
pub async fn get_character_logs(
    configuration: &configuration::Configuration,
    params: GetCharacterLogsParams,
) -> Result<models::DataPageLogSchema, Error<GetCharacterLogsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/logs/{name}",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
    );
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
        let local_var_entity: Option<GetCharacterLogsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List of your characters.
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
        let local_var_entity: Option<GetMyCharactersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Give gold to another character in your account on the same map.
pub async fn give_gold(
    configuration: &configuration::Configuration,
    params: GiveGoldParams,
) -> Result<models::GiveGoldResponseSchema, Error<GiveGoldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let give_gold_schema = params.give_gold_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/give/gold",
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
    local_var_req_builder = local_var_req_builder.json(&give_gold_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GiveGoldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Give items to another character in your account on the same map. The cooldown will be 3 seconds multiplied by the number of different items given.
pub async fn give_items(
    configuration: &configuration::Configuration,
    params: GiveItemsParams,
) -> Result<models::GiveItemResponseSchema, Error<GiveItemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let give_items_schema = params.give_items_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/my/{name}/action/give/item",
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
    local_var_req_builder = local_var_req_builder.json(&give_items_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GiveItemsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Moves a character on the map using either the map's ID or X and Y position. Provide either 'map_id' or both 'x' and 'y' coordinates in the request body.
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
        let local_var_entity: Option<MoveCharacterError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<NpcBuyItemError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<NpcSellItemError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<RecycleError> = serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<RestCharacterError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<TaskExchangeError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<TaskTradeError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<UnequipItemError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<UseItemError> = serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<WithdrawGoldError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Take items from your bank and put them in the character's inventory. The cooldown will be 3 seconds multiplied by the number of different items withdrawn.
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
        "{}/my/{name}/action/bank/withdraw/item",
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
        let local_var_entity: Option<WithdrawItemError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
