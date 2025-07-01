use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`create_account`]
#[derive(Clone, Debug)]
pub struct CreateAccountParams {
    pub add_account_schema: models::AddAccountSchema,
}

impl CreateAccountParams {
    pub fn new(add_account_schema: models::AddAccountSchema) -> Self {
        Self { add_account_schema }
    }
}

/// struct for passing parameters to the method [`forgot_password`]
#[derive(Clone, Debug)]
pub struct ForgotPasswordParams {
    pub password_reset_request_schema: models::PasswordResetRequestSchema,
}

impl ForgotPasswordParams {
    pub fn new(password_reset_request_schema: models::PasswordResetRequestSchema) -> Self {
        Self {
            password_reset_request_schema,
        }
    }
}

/// struct for passing parameters to the method [`get_account`]
#[derive(Clone, Debug)]
pub struct GetAccountParams {
    /// The account name.
    pub account: String,
}

impl GetAccountParams {
    pub fn new(account: String) -> Self {
        Self { account }
    }
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

impl GetAccountAchievementsParams {
    pub fn new(
        account: String,
        r#type: Option<String>,
        completed: Option<bool>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            account,
            r#type,
            completed,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_account_characters`]
#[derive(Clone, Debug)]
pub struct GetAccountCharactersParams {
    /// The character name.
    pub account: String,
}

impl GetAccountCharactersParams {
    pub fn new(account: String) -> Self {
        Self { account }
    }
}

/// struct for passing parameters to the method [`reset_password`]
#[derive(Clone, Debug)]
pub struct ResetPasswordParams {
    pub password_reset_confirm_schema: models::PasswordResetConfirmSchema,
}

impl ResetPasswordParams {
    pub fn new(password_reset_confirm_schema: models::PasswordResetConfirmSchema) -> Self {
        Self {
            password_reset_confirm_schema,
        }
    }
}

/// struct for typed errors of method [`create_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAccountError {
    /// This username is already taken.
    Status456,
    /// This email is already in use.
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

/// struct for typed errors of method [`forgot_password`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForgotPasswordError {}

impl TryFrom<StatusCode> for ForgotPasswordError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
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

/// struct for typed errors of method [`reset_password`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetPasswordError {
    /// The password reset token has expired.
    Status561,
    /// This password reset token has already been used.
    Status562,
    /// The password reset token is invalid.
    Status560,
}

impl TryFrom<StatusCode> for ResetPasswordError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            561 => Ok(Self::Status561),
            562 => Ok(Self::Status562),
            560 => Ok(Self::Status560),
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

/// Request a password reset.
pub async fn forgot_password(
    configuration: &configuration::Configuration,
    params: ForgotPasswordParams,
) -> Result<models::PasswordResetResponseSchema, Error<ForgotPasswordError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let password_reset_request_schema = params.password_reset_request_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/accounts/forgot_password",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&password_reset_request_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ForgotPasswordError> = local_var_status.try_into().ok();
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

/// Reset password with a token. Use /forgot_password to get a token by email.
pub async fn reset_password(
    configuration: &configuration::Configuration,
    params: ResetPasswordParams,
) -> Result<models::PasswordResetResponseSchema, Error<ResetPasswordError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let password_reset_confirm_schema = params.password_reset_confirm_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/accounts/reset_password",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&password_reset_confirm_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ResetPasswordError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
