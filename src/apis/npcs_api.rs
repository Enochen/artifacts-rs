use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_all_npcs_npcs_get`]
#[derive(Clone, Debug)]
pub struct GetAllNpcsNpcsGetParams {
    /// The type of the NPC.
    pub r#type: Option<models::NpcType>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_npc_items_npcs_code_items_get`]
#[derive(Clone, Debug)]
pub struct GetNpcItemsNpcsCodeItemsGetParams {
    /// The code of the NPC.
    pub code: String,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_npc_npcs_code_get`]
#[derive(Clone, Debug)]
pub struct GetNpcNpcsCodeGetParams {
    /// The code of the NPC.
    pub code: String,
}

/// struct for typed errors of method [`get_all_npcs_npcs_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllNpcsNpcsGetError {}

impl TryFrom<StatusCode> for GetAllNpcsNpcsGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_npc_items_npcs_code_items_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNpcItemsNpcsCodeItemsGetError {
    /// NPC not found.
    Status404,
}

impl TryFrom<StatusCode> for GetNpcItemsNpcsCodeItemsGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_npc_npcs_code_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNpcNpcsCodeGetError {
    /// NPC not found.
    Status404,
}

impl TryFrom<StatusCode> for GetNpcNpcsCodeGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// Fetch NPCs details.
pub async fn get_all_npcs_npcs_get(
    configuration: &configuration::Configuration,
    params: GetAllNpcsNpcsGetParams,
) -> Result<models::DataPageNpcSchema, Error<GetAllNpcsNpcsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/npcs", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllNpcsNpcsGetError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the items list of a NPC. If the NPC has items to buy or sell, they will be displayed.
pub async fn get_npc_items_npcs_code_items_get(
    configuration: &configuration::Configuration,
    params: GetNpcItemsNpcsCodeItemsGetParams,
) -> Result<models::DataPageNpcItem, Error<GetNpcItemsNpcsCodeItemsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/npcs/{code}/items",
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
        let local_var_entity: Option<GetNpcItemsNpcsCodeItemsGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a NPC.
pub async fn get_npc_npcs_code_get(
    configuration: &configuration::Configuration,
    params: GetNpcNpcsCodeGetParams,
) -> Result<models::NpcResponseSchema, Error<GetNpcNpcsCodeGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/npcs/{code}",
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
        let local_var_entity: Option<GetNpcNpcsCodeGetError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
