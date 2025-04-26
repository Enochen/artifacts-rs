use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_all_maps`]
#[derive(Clone, Debug)]
pub struct GetAllMapsParams {
    /// Type of content on the map.
    pub content_type: Option<models::MapContentType>,
    /// Content code on the map.
    pub content_code: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllMapsParams {
    pub fn new(
        content_type: Option<models::MapContentType>,
        content_code: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            content_type,
            content_code,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_map`]
#[derive(Clone, Debug)]
pub struct GetMapParams {
    /// The position x of the map.
    pub x: i32,
    /// The position X of the map.
    pub y: i32,
}

impl GetMapParams {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// struct for typed errors of method [`get_all_maps`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllMapsError {}

impl TryFrom<StatusCode> for GetAllMapsError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_map`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMapError {
    /// Map not found.
    Status404,
}

impl TryFrom<StatusCode> for GetMapError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// Fetch maps details.
pub async fn get_all_maps(
    configuration: &configuration::Configuration,
    params: GetAllMapsParams,
) -> Result<models::DataPageMapSchema, Error<GetAllMapsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let content_type = params.content_type;
    // unbox the parameters
    let content_code = params.content_code;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/maps", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = content_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("content_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = content_code {
        local_var_req_builder =
            local_var_req_builder.query(&[("content_code", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllMapsError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a map.
pub async fn get_map(
    configuration: &configuration::Configuration,
    params: GetMapParams,
) -> Result<models::MapResponseSchema, Error<GetMapError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let x = params.x;
    // unbox the parameters
    let y = params.y;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/maps/{x}/{y}",
        local_var_configuration.base_path,
        x = x,
        y = y
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
        let local_var_entity: Option<GetMapError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
