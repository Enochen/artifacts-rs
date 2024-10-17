use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`create_character`]
#[derive(Clone, Debug)]
pub struct CreateCharacterParams {
    pub add_character_schema: models::AddCharacterSchema,
}

/// struct for passing parameters to the method [`delete_character`]
#[derive(Clone, Debug)]
pub struct DeleteCharacterParams {
    pub delete_character_schema: models::DeleteCharacterSchema,
}

/// struct for passing parameters to the method [`get_all_characters`]
#[derive(Clone, Debug)]
pub struct GetAllCharactersParams {
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_character`]
#[derive(Clone, Debug)]
pub struct GetCharacterParams {
    /// The character name.
    pub name: String,
}

/// struct for passing parameters to the method [`get_character_achievements`]
#[derive(Clone, Debug)]
pub struct GetCharacterAchievementsParams {
    /// The character name.
    pub name: String,
    /// Type of achievements.
    pub r#type: Option<String>,
    /// Filter by completed achievements.
    pub completed: Option<bool>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for typed errors of method [`create_character`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCharacterError {
    /// Name already used.
    Status494,
    /// Maximum characters reached on your account.
    Status495,
}

impl TryFrom<StatusCode> for CreateCharacterError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            494 => Ok(Self::Status494),
            495 => Ok(Self::Status495),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`delete_character`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCharacterError {
    /// Character not found.
    Status498,
}

impl TryFrom<StatusCode> for DeleteCharacterError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            498 => Ok(Self::Status498),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_all_characters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllCharactersError {}

impl TryFrom<StatusCode> for GetAllCharactersError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_character`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharacterError {
    /// Character not found.
    Status404,
}

impl TryFrom<StatusCode> for GetCharacterError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_character_achievements`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCharacterAchievementsError {
    /// Character not found.
    Status404,
}

impl TryFrom<StatusCode> for GetCharacterAchievementsError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// Create new character on your account. You can create up to 5 characters.
pub async fn create_character(
    configuration: &configuration::Configuration,
    params: CreateCharacterParams,
) -> Result<models::CharacterResponseSchema, Error<CreateCharacterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let add_character_schema = params.add_character_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/create", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&add_character_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCharacterError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete character on your account.
pub async fn delete_character(
    configuration: &configuration::Configuration,
    params: DeleteCharacterParams,
) -> Result<models::CharacterResponseSchema, Error<DeleteCharacterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let delete_character_schema = params.delete_character_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/delete", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&delete_character_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteCharacterError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch characters details.
pub async fn get_all_characters(
    configuration: &configuration::Configuration,
    params: GetAllCharactersParams,
) -> Result<models::DataPageCharacterSchema, Error<GetAllCharactersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetAllCharactersError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a character.
pub async fn get_character(
    configuration: &configuration::Configuration,
    params: GetCharacterParams,
) -> Result<models::CharacterResponseSchema, Error<GetCharacterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/characters/{name}",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
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
        let local_var_entity: Option<GetCharacterError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a character.
pub async fn get_character_achievements(
    configuration: &configuration::Configuration,
    params: GetCharacterAchievementsParams,
) -> Result<models::DataPageAchievementSchema, Error<GetCharacterAchievementsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
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
        "{}/characters/{name}/achievements",
        local_var_configuration.base_path,
        name = crate::apis::urlencode(name)
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
        let local_var_entity: Option<GetCharacterAchievementsError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
