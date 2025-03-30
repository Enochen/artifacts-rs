use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_all_effects_effects_get`]
#[derive(Clone, Debug)]
pub struct GetAllEffectsEffectsGetParams {
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_effect_effects_code_get`]
#[derive(Clone, Debug)]
pub struct GetEffectEffectsCodeGetParams {
    /// The code of the achievement.
    pub code: String,
}

/// struct for typed errors of method [`get_all_effects_effects_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllEffectsEffectsGetError {}

impl TryFrom<StatusCode> for GetAllEffectsEffectsGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_effect_effects_code_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEffectEffectsCodeGetError {
    /// effect not found.
    Status404,
}

impl TryFrom<StatusCode> for GetEffectEffectsCodeGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// List of all effects. Effects are used by equipment, tools, runes, consumables and monsters. An effect is an action that produces an effect on the game.
pub async fn get_all_effects_effects_get(
    configuration: &configuration::Configuration,
    params: GetAllEffectsEffectsGetParams,
) -> Result<models::DataPageEffectSchema, Error<GetAllEffectsEffectsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/effects", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetAllEffectsEffectsGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a badge.
pub async fn get_effect_effects_code_get(
    configuration: &configuration::Configuration,
    params: GetEffectEffectsCodeGetParams,
) -> Result<models::EffectResponseSchema, Error<GetEffectEffectsCodeGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/effects/{code}",
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
        let local_var_entity: Option<GetEffectEffectsCodeGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
