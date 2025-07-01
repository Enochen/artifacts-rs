use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_all_items`]
#[derive(Clone, Debug)]
pub struct GetAllItemsParams {
    /// Name of the item.
    pub name: Option<String>,
    /// Minimum level items.
    pub min_level: Option<u32>,
    /// Maximum level items.
    pub max_level: Option<u32>,
    /// Type of items.
    pub r#type: Option<models::ItemType>,
    /// Skill to craft items.
    pub craft_skill: Option<models::CraftSkill>,
    /// Item code of items used as material for crafting.
    pub craft_material: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllItemsParams {
    pub fn new(
        name: Option<String>,
        min_level: Option<u32>,
        max_level: Option<u32>,
        r#type: Option<models::ItemType>,
        craft_skill: Option<models::CraftSkill>,
        craft_material: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            name,
            min_level,
            max_level,
            r#type,
            craft_skill,
            craft_material,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_item`]
#[derive(Clone, Debug)]
pub struct GetItemParams {
    /// The code of the item.
    pub code: String,
}

impl GetItemParams {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// struct for typed errors of method [`get_all_items`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllItemsError {}

impl TryFrom<StatusCode> for GetAllItemsError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_item`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetItemError {
    /// Item not found.
    Status404,
}

impl TryFrom<StatusCode> for GetItemError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// Fetch items details.
pub async fn get_all_items(
    configuration: &configuration::Configuration,
    params: GetAllItemsParams,
) -> Result<models::DataPageItemSchema, Error<GetAllItemsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let min_level = params.min_level;
    // unbox the parameters
    let max_level = params.max_level;
    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let craft_skill = params.craft_skill;
    // unbox the parameters
    let craft_material = params.craft_material;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/items", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("min_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("max_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = craft_skill {
        local_var_req_builder =
            local_var_req_builder.query(&[("craft_skill", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = craft_material {
        local_var_req_builder =
            local_var_req_builder.query(&[("craft_material", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllItemsError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a item.
pub async fn get_item(
    configuration: &configuration::Configuration,
    params: GetItemParams,
) -> Result<models::ItemResponseSchema, Error<GetItemError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/items/{code}",
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
        let local_var_entity: Option<GetItemError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
