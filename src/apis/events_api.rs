use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_all_active_events_events_active_get`]
#[derive(Clone, Debug)]
pub struct GetAllActiveEventsEventsActiveGetParams {
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_all_events`]
#[derive(Clone, Debug)]
pub struct GetAllEventsParams {
    /// Type of event.
    pub r#type: Option<models::MapContentType>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for typed errors of method [`get_all_active_events_events_active_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllActiveEventsEventsActiveGetError {}

impl TryFrom<StatusCode> for GetAllActiveEventsEventsActiveGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_all_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllEventsError {}

impl TryFrom<StatusCode> for GetAllEventsError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// Fetch active events details.
pub async fn get_all_active_events_events_active_get(
    configuration: &configuration::Configuration,
    params: GetAllActiveEventsEventsActiveGetParams,
) -> Result<models::DataPageActiveEventSchema, Error<GetAllActiveEventsEventsActiveGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/events/active", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetAllActiveEventsEventsActiveGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch events details.
pub async fn get_all_events(
    configuration: &configuration::Configuration,
    params: GetAllEventsParams,
) -> Result<models::DataPageEventSchema, Error<GetAllEventsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/events", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetAllEventsError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
