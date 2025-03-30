use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`create_account`]
#[derive(Clone, Debug)]
pub struct CreateAccountParams {
    pub add_account_schema: models::AddAccountSchema,
}

/// struct for passing parameters to the method [`get_account`]
#[derive(Clone, Debug)]
pub struct GetAccountParams {
    /// The account name.
    pub account: String,
}

/// struct for passing parameters to the method [`get_account_achievements`]
#[derive(Clone, Debug)]
pub struct GetAccountAchievementsParams {
    /// The character name.
    pub account: String,
    /// Type of achievements.
    pub r#type: Option<String>,
    /// Filter by completed achievements.
    pub completed: Option<bool>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_account_characters`]
#[derive(Clone, Debug)]
pub struct GetAccountCharactersParams {
    /// The character name.
    pub account: String,
}

/// struct for typed errors of method [`create_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAccountError {
    /// Username already used.
    Status456,
    /// Email already used.
    Status457,
}

impl TryFrom<StatusCode> for CreateAccountError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            456 => Ok(Self::Status456),
            457 => Ok(Self::Status457),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountError {
    /// Account not found.
    Status404,
}

impl TryFrom<StatusCode> for GetAccountError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_account_achievements`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountAchievementsError {
    /// Account not found.
    Status404,
}

impl TryFrom<StatusCode> for GetAccountAchievementsError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_account_characters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountCharactersError {}

impl TryFrom<StatusCode> for GetAccountCharactersError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

pub async fn create_account(
    configuration: &configuration::Configuration,
    params: CreateAccountParams,
) -> Result<models::ResponseSchema, Error<CreateAccountError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let add_account_schema = params.add_account_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/accounts/create", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&add_account_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAccountError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a character.
pub async fn get_account(
    configuration: &configuration::Configuration,
    params: GetAccountParams,
) -> Result<models::AccountDetailsSchema, Error<GetAccountError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account = params.account;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/accounts/{account}",
        local_var_configuration.base_path,
        account = crate::apis::urlencode(account)
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
        let local_var_entity: Option<GetAccountError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the achievements of a account.
pub async fn get_account_achievements(
    configuration: &configuration::Configuration,
    params: GetAccountAchievementsParams,
) -> Result<models::DataPageAccountAchievementSchema, Error<GetAccountAchievementsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account = params.account;
    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let completed = params.completed;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/accounts/{account}/achievements",
        local_var_configuration.base_path,
        account = crate::apis::urlencode(account)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = completed {
        local_var_req_builder =
            local_var_req_builder.query(&[("completed", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAccountAchievementsError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Account character lists.
pub async fn get_account_characters(
    configuration: &configuration::Configuration,
    params: GetAccountCharactersParams,
) -> Result<models::CharactersListSchema, Error<GetAccountCharactersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account = params.account;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/accounts/{account}/characters",
        local_var_configuration.base_path,
        account = crate::apis::urlencode(account)
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
        let local_var_entity: Option<GetAccountCharactersError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
