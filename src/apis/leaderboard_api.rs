use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_accounts_leaderboard`]
#[derive(Clone, Debug)]
pub struct GetAccountsLeaderboardParams {
    /// Sort of account leaderboards.
    pub sort: Option<models::AccountLeaderboardType>,
    /// Account name.
    pub name: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAccountsLeaderboardParams {
    pub fn new(
        sort: Option<models::AccountLeaderboardType>,
        name: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            sort,
            name,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_characters_leaderboard`]
#[derive(Clone, Debug)]
pub struct GetCharactersLeaderboardParams {
    /// Sort of character leaderboards.
    pub sort: Option<models::CharacterLeaderboardType>,
    /// Character name.
    pub name: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetCharactersLeaderboardParams {
    pub fn new(
        sort: Option<models::CharacterLeaderboardType>,
        name: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            sort,
            name,
            page,
            size,
        }
    }
}

/// struct for typed errors of method [`get_accounts_leaderboard`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetAccountsLeaderboardError {}

impl<'de> Deserialize<'de> for GetAccountsLeaderboardError {
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

/// struct for typed errors of method [`get_characters_leaderboard`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetCharactersLeaderboardError {}

impl<'de> Deserialize<'de> for GetCharactersLeaderboardError {
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

/// Fetch leaderboard details.
pub async fn get_accounts_leaderboard(
    configuration: &configuration::Configuration,
    params: GetAccountsLeaderboardParams,
) -> Result<models::DataPageAccountLeaderboardSchema, Error<GetAccountsLeaderboardError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sort = params.sort;
    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/leaderboard/accounts", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAccountsLeaderboardError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch leaderboard details.
pub async fn get_characters_leaderboard(
    configuration: &configuration::Configuration,
    params: GetCharactersLeaderboardParams,
) -> Result<models::DataPageCharacterLeaderboardSchema, Error<GetCharactersLeaderboardError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sort = params.sort;
    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/leaderboard/characters",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetCharactersLeaderboardError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
