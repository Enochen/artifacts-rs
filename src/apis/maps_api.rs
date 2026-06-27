use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_all_maps`]
#[derive(Clone, Debug)]
pub struct GetAllMapsParams {
    /// Filter maps by layer.
    pub layer: Option<models::MapLayer>,
    /// Type of maps.
    pub content_type: Option<models::MapContentType>,
    /// Content code on the map.
    pub content_code: Option<String>,
    /// When true, excludes maps with access_type 'blocked' from the results.
    pub hide_blocked_maps: Option<bool>,
    /// When true, does not overlay active events on maps.
    pub hide_event: Option<bool>,
    /// Filter maps by transition. True returns only maps with transitions, False returns only maps without.
    pub transition: Option<bool>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllMapsParams {
    pub fn new(
        layer: Option<models::MapLayer>,
        content_type: Option<models::MapContentType>,
        content_code: Option<String>,
        hide_blocked_maps: Option<bool>,
        hide_event: Option<bool>,
        transition: Option<bool>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            layer,
            content_type,
            content_code,
            hide_blocked_maps,
            hide_event,
            transition,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_layer_maps`]
#[derive(Clone, Debug)]
pub struct GetLayerMapsParams {
    /// The layer of the map (interior, overworld, underground).
    pub layer: models::MapLayer,
    /// Type of maps.
    pub content_type: Option<models::MapContentType>,
    /// Content code on the map.
    pub content_code: Option<String>,
    /// When true, excludes maps with access_type 'blocked' from the results.
    pub hide_blocked_maps: Option<bool>,
    /// When true, does not overlay active events on maps.
    pub hide_event: Option<bool>,
    /// Filter maps by transition. True returns only maps with transitions, False returns only maps without.
    pub transition: Option<bool>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetLayerMapsParams {
    pub fn new(
        layer: models::MapLayer,
        content_type: Option<models::MapContentType>,
        content_code: Option<String>,
        hide_blocked_maps: Option<bool>,
        hide_event: Option<bool>,
        transition: Option<bool>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            layer,
            content_type,
            content_code,
            hide_blocked_maps,
            hide_event,
            transition,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_map_by_id`]
#[derive(Clone, Debug)]
pub struct GetMapByIdParams {
    /// The unique ID of the map.
    pub map_id: i32,
}

impl GetMapByIdParams {
    pub fn new(map_id: i32) -> Self {
        Self { map_id }
    }
}

/// struct for passing parameters to the method [`get_map_by_position`]
#[derive(Clone, Debug)]
pub struct GetMapByPositionParams {
    /// The layer of the map (interior, overworld, underground).
    pub layer: models::MapLayer,
    /// The position x of the map.
    pub x: i32,
    /// The position y of the map.
    pub y: i32,
}

impl GetMapByPositionParams {
    pub fn new(layer: models::MapLayer, x: i32, y: i32) -> Self {
        Self { layer, x, y }
    }
}

/// struct for typed errors of method [`get_all_maps`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllMapsError {}

impl<'de> Deserialize<'de> for GetAllMapsError {
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

/// struct for typed errors of method [`get_layer_maps`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetLayerMapsError {}

impl<'de> Deserialize<'de> for GetLayerMapsError {
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

/// struct for typed errors of method [`get_map_by_id`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetMapByIdError {
    /// map not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetMapByIdError {
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

/// struct for typed errors of method [`get_map_by_position`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetMapByPositionError {
    /// map not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetMapByPositionError {
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

/// Fetch maps details.
pub async fn get_all_maps(
    configuration: &configuration::Configuration,
    params: GetAllMapsParams,
) -> Result<models::StaticDataPageMapSchema, Error<GetAllMapsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let layer = params.layer;
    // unbox the parameters
    let content_type = params.content_type;
    // unbox the parameters
    let content_code = params.content_code;
    // unbox the parameters
    let hide_blocked_maps = params.hide_blocked_maps;
    // unbox the parameters
    let hide_event = params.hide_event;
    // unbox the parameters
    let transition = params.transition;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/maps", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = layer {
        local_var_req_builder =
            local_var_req_builder.query(&[("layer", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = content_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("content_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = content_code {
        local_var_req_builder =
            local_var_req_builder.query(&[("content_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = hide_blocked_maps {
        local_var_req_builder =
            local_var_req_builder.query(&[("hide_blocked_maps", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = hide_event {
        local_var_req_builder =
            local_var_req_builder.query(&[("hide_event", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transition {
        local_var_req_builder =
            local_var_req_builder.query(&[("transition", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllMapsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch maps details.
pub async fn get_layer_maps(
    configuration: &configuration::Configuration,
    params: GetLayerMapsParams,
) -> Result<models::StaticDataPageMapSchema, Error<GetLayerMapsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let layer = params.layer;
    // unbox the parameters
    let content_type = params.content_type;
    // unbox the parameters
    let content_code = params.content_code;
    // unbox the parameters
    let hide_blocked_maps = params.hide_blocked_maps;
    // unbox the parameters
    let hide_event = params.hide_event;
    // unbox the parameters
    let transition = params.transition;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/maps/{layer}",
        local_var_configuration.base_path,
        layer = layer
    );
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
    if let Some(ref local_var_str) = hide_blocked_maps {
        local_var_req_builder =
            local_var_req_builder.query(&[("hide_blocked_maps", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = hide_event {
        local_var_req_builder =
            local_var_req_builder.query(&[("hide_event", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = transition {
        local_var_req_builder =
            local_var_req_builder.query(&[("transition", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetLayerMapsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a map by its unique ID.
pub async fn get_map_by_id(
    configuration: &configuration::Configuration,
    params: GetMapByIdParams,
) -> Result<models::MapResponseSchema, Error<GetMapByIdError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let map_id = params.map_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/maps/id/{map_id}",
        local_var_configuration.base_path,
        map_id = map_id
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
        let local_var_entity: Option<GetMapByIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a map by layer and coordinates.
pub async fn get_map_by_position(
    configuration: &configuration::Configuration,
    params: GetMapByPositionParams,
) -> Result<models::MapResponseSchema, Error<GetMapByPositionError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let layer = params.layer;
    // unbox the parameters
    let x = params.x;
    // unbox the parameters
    let y = params.y;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/maps/{layer}/{x}/{y}",
        local_var_configuration.base_path,
        layer = layer,
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
        let local_var_entity: Option<GetMapByPositionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
