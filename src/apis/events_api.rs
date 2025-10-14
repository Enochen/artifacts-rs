use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_all_active_events`]
#[derive(Clone, Debug)]
pub struct GetAllActiveEventsParams {
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllActiveEventsParams {
    pub fn new(page: Option<u32>, size: Option<u32>) -> Self {
        Self { page, size }
    }
}

/// struct for passing parameters to the method [`get_all_events`]
#[derive(Clone, Debug)]
pub struct GetAllEventsParams {
    /// Type of events.
    pub r#type: Option<models::MapContentType>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllEventsParams {
    pub fn new(
        r#type: Option<models::MapContentType>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self { r#type, page, size }
    }
}

/// struct for passing parameters to the method [`spawn_event`]
#[derive(Clone, Debug)]
pub struct SpawnEventParams {
    pub spawn_event_request: models::SpawnEventRequest,
}

impl SpawnEventParams {
    pub fn new(spawn_event_request: models::SpawnEventRequest) -> Self {
        Self {
            spawn_event_request,
        }
    }
}

/// struct for typed errors of method [`get_all_active_events`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllActiveEventsError {}

impl<'de> Deserialize<'de> for GetAllActiveEventsError {
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

/// struct for typed errors of method [`get_all_events`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllEventsError {}

impl<'de> Deserialize<'de> for GetAllEventsError {
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

/// struct for typed errors of method [`spawn_event`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SpawnEventError {
    /// Insufficient event tokens. You need at least 1 event token to spawn an event.
    Status563(models::ErrorResponseSchema),
    /// Event not found or already active.
    Status564(models::ErrorResponseSchema),
    /// Access denied, you must be a member to do that.
    Status451(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for SpawnEventError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            563 => Ok(Self::Status563(raw)),
            564 => Ok(Self::Status564(raw)),
            451 => Ok(Self::Status451(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// Fetch active events details.
pub async fn get_all_active_events(
    configuration: &configuration::Configuration,
    params: GetAllActiveEventsParams,
) -> Result<models::DataPageActiveEventSchema, Error<GetAllActiveEventsError>> {
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
        let local_var_entity: Option<GetAllActiveEventsError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<GetAllEventsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Spawn a specific event by code consuming 1 event token.  Rules:   - Maximum active events defined by utils.config.max_active_events().   - Event must not already be active.   - Member or founder account required.
pub async fn spawn_event(
    configuration: &configuration::Configuration,
    params: SpawnEventParams,
) -> Result<models::ActiveEventResponseSchema, Error<SpawnEventError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let spawn_event_request = params.spawn_event_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/events/spawn", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&spawn_event_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SpawnEventError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
