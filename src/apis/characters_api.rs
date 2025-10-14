use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`create_character`]
#[derive(Clone, Debug)]
pub struct CreateCharacterParams {
    pub add_character_schema: models::AddCharacterSchema,
}

impl CreateCharacterParams {
    pub fn new(add_character_schema: models::AddCharacterSchema) -> Self {
        Self {
            add_character_schema,
        }
    }
}

/// struct for passing parameters to the method [`delete_character`]
#[derive(Clone, Debug)]
pub struct DeleteCharacterParams {
    pub delete_character_schema: models::DeleteCharacterSchema,
}

impl DeleteCharacterParams {
    pub fn new(delete_character_schema: models::DeleteCharacterSchema) -> Self {
        Self {
            delete_character_schema,
        }
    }
}

/// struct for passing parameters to the method [`get_active_characters`]
#[derive(Clone, Debug)]
pub struct GetActiveCharactersParams {
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetActiveCharactersParams {
    pub fn new(page: Option<u32>, size: Option<u32>) -> Self {
        Self { page, size }
    }
}

/// struct for passing parameters to the method [`get_character`]
#[derive(Clone, Debug)]
pub struct GetCharacterParams {
    /// The name of the character.
    pub name: String,
}

impl GetCharacterParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// struct for typed errors of method [`create_character`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CreateCharacterError {
    /// This name is already in use.
    Status494(models::ErrorResponseSchema),
    /// You have reached the maximum number of characters on your account.
    Status495(models::ErrorResponseSchema),
    /// You cannot choose this skin because you do not own it.
    Status550(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for CreateCharacterError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            494 => Ok(Self::Status494(raw)),
            495 => Ok(Self::Status495(raw)),
            550 => Ok(Self::Status550(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`delete_character`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DeleteCharacterError {
    /// Character not found.
    Status498(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for DeleteCharacterError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            498 => Ok(Self::Status498(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`get_active_characters`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetActiveCharactersError {}

impl<'de> Deserialize<'de> for GetActiveCharactersError {
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

/// struct for typed errors of method [`get_character`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetCharacterError {
    /// character not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetCharacterError {
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
        let local_var_entity: Option<CreateCharacterError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<DeleteCharacterError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch active characters details.
pub async fn get_active_characters(
    configuration: &configuration::Configuration,
    params: GetActiveCharactersParams,
) -> Result<models::DataPageActiveCharacterSchema, Error<GetActiveCharactersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/characters/active", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetActiveCharactersError> =
            serde_json::from_str(&local_var_content).ok();
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
        let local_var_entity: Option<GetCharacterError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
