use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_all_npc_items`]
#[derive(Clone, Debug)]
pub struct GetAllNpcItemsParams {
    /// Item code.
    pub code: Option<String>,
    /// NPC code.
    pub npc: Option<String>,
    /// Currency code.
    pub currency: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllNpcItemsParams {
    pub fn new(
        code: Option<String>,
        npc: Option<String>,
        currency: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            code,
            npc,
            currency,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_all_npcs`]
#[derive(Clone, Debug)]
pub struct GetAllNpcsParams {
    /// NPC name.
    pub name: Option<String>,
    /// Type of NPCs.
    pub r#type: Option<models::NpcType>,
    /// Currency code to filter NPCs that trade with this currency.
    pub currency: Option<String>,
    /// Item code to filter NPCs that trade this item.
    pub item: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllNpcsParams {
    pub fn new(
        name: Option<String>,
        r#type: Option<models::NpcType>,
        currency: Option<String>,
        item: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            name,
            r#type,
            currency,
            item,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_npc`]
#[derive(Clone, Debug)]
pub struct GetNpcParams {
    /// The code of the NPC.
    pub code: String,
}

impl GetNpcParams {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// struct for passing parameters to the method [`get_npc_items`]
#[derive(Clone, Debug)]
pub struct GetNpcItemsParams {
    /// The code of the NPC.
    pub code: String,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetNpcItemsParams {
    pub fn new(code: String, page: Option<u32>, size: Option<u32>) -> Self {
        Self { code, page, size }
    }
}

/// struct for typed errors of method [`get_all_npc_items`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllNpcItemsError {}

impl<'de> Deserialize<'de> for GetAllNpcItemsError {
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

/// struct for typed errors of method [`get_all_npcs`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllNpcsError {}

impl<'de> Deserialize<'de> for GetAllNpcsError {
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

/// struct for typed errors of method [`get_npc`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetNpcError {
    /// NPC not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetNpcError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`get_npc_items`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetNpcItemsError {
    /// NPC items not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetNpcItemsError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// Retrieve the list of all NPC items.
pub async fn get_all_npc_items(
    configuration: &configuration::Configuration,
    params: GetAllNpcItemsParams,
) -> Result<models::StaticDataPageNpcItemSchema, Error<GetAllNpcItemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let npc = params.npc;
    // unbox the parameters
    let currency = params.currency;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/npcs/items", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = code {
        local_var_req_builder =
            local_var_req_builder.query(&[("code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = npc {
        local_var_req_builder = local_var_req_builder.query(&[("npc", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = currency {
        local_var_req_builder =
            local_var_req_builder.query(&[("currency", &local_var_str.to_string())]);
    }
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllNpcItemsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch NPCs details.
pub async fn get_all_npcs(
    configuration: &configuration::Configuration,
    params: GetAllNpcsParams,
) -> Result<models::StaticDataPageNpcSchema, Error<GetAllNpcsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let currency = params.currency;
    // unbox the parameters
    let item = params.item;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/npcs/details", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = currency {
        local_var_req_builder =
            local_var_req_builder.query(&[("currency", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = item {
        local_var_req_builder =
            local_var_req_builder.query(&[("item", &local_var_str.to_string())]);
    }
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAllNpcsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a NPC.
pub async fn get_npc(
    configuration: &configuration::Configuration,
    params: GetNpcParams,
) -> Result<models::NpcResponseSchema, Error<GetNpcError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/npcs/details/{code}",
        local_var_configuration.base_path,
        code = crate::apis::urlencode(code)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetNpcError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the items list of a NPC. If the NPC has items to buy, sell or trade, they will be displayed.
pub async fn get_npc_items(
    configuration: &configuration::Configuration,
    params: GetNpcItemsParams,
) -> Result<models::StaticDataPageNpcItemSchema, Error<GetNpcItemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/npcs/items/{code}",
        local_var_configuration.base_path,
        code = crate::apis::urlencode(code)
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetNpcItemsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
