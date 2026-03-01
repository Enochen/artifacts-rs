use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_all_resources`]
#[derive(Clone, Debug)]
pub struct GetAllResourcesParams {
    /// Minimum level.
    pub min_level: Option<u32>,
    /// Maximum level.
    pub max_level: Option<u32>,
    /// Skill of resources.
    pub skill: Option<models::GatheringSkill>,
    /// Item code of the drop.
    pub drop: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllResourcesParams {
    pub fn new(
        min_level: Option<u32>,
        max_level: Option<u32>,
        skill: Option<models::GatheringSkill>,
        drop: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            min_level,
            max_level,
            skill,
            drop,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_resource`]
#[derive(Clone, Debug)]
pub struct GetResourceParams {
    /// The code of the resource.
    pub code: String,
}

impl GetResourceParams {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// struct for typed errors of method [`get_all_resources`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllResourcesError {}

impl<'de> Deserialize<'de> for GetAllResourcesError {
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

/// struct for typed errors of method [`get_resource`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetResourceError {
    /// resource not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetResourceError {
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

/// Fetch resources details.
pub async fn get_all_resources(
    configuration: &configuration::Configuration,
    params: GetAllResourcesParams,
) -> Result<models::StaticDataPageResourceSchema, Error<GetAllResourcesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let min_level = params.min_level;
    // unbox the parameters
    let max_level = params.max_level;
    // unbox the parameters
    let skill = params.skill;
    // unbox the parameters
    let drop = params.drop;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/resources", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = min_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("min_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("max_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = skill {
        local_var_req_builder =
            local_var_req_builder.query(&[("skill", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = drop {
        local_var_req_builder =
            local_var_req_builder.query(&[("drop", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllResourcesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a resource.
pub async fn get_resource(
    configuration: &configuration::Configuration,
    params: GetResourceParams,
) -> Result<models::ResourceResponseSchema, Error<GetResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/resources/{code}",
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
        let local_var_entity: Option<GetResourceError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
