use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_all_season_rewards`]
#[derive(Clone, Debug)]
pub struct GetAllSeasonRewardsParams {
    /// Filter by reward type.
    pub r#type: Option<models::RewardType>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllSeasonRewardsParams {
    pub fn new(r#type: Option<models::RewardType>, page: Option<u32>, size: Option<u32>) -> Self {
        Self { r#type, page, size }
    }
}

/// struct for passing parameters to the method [`get_season_rewards_by_code`]
#[derive(Clone, Debug)]
pub struct GetSeasonRewardsByCodeParams {
    /// The code of the season reward.
    pub code: String,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetSeasonRewardsByCodeParams {
    pub fn new(code: String, page: Option<u32>, size: Option<u32>) -> Self {
        Self { code, page, size }
    }
}

/// struct for typed errors of method [`get_all_season_rewards`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAllSeasonRewardsError {}

impl<'de> Deserialize<'de> for GetAllSeasonRewardsError {
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

/// struct for typed errors of method [`get_season_rewards_by_code`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetSeasonRewardsByCodeError {}

impl<'de> Deserialize<'de> for GetSeasonRewardsByCodeError {
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

/// List of all rewards for the current season.
pub async fn get_all_season_rewards(
    configuration: &configuration::Configuration,
    params: GetAllSeasonRewardsParams,
) -> Result<models::StaticDataPageSeasonRewardSchema, Error<GetAllSeasonRewardsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/season_rewards", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetAllSeasonRewardsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all season rewards matching a specific code.
pub async fn get_season_rewards_by_code(
    configuration: &configuration::Configuration,
    params: GetSeasonRewardsByCodeParams,
) -> Result<models::StaticDataPageSeasonRewardSchema, Error<GetSeasonRewardsByCodeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/season_rewards/{code}",
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
        let local_var_entity: Option<GetSeasonRewardsByCodeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
