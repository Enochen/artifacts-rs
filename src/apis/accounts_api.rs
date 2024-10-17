use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`create_account`]
#[derive(Clone, Debug)]
pub struct CreateAccountParams {
    pub add_account_schema: models::AddAccountSchema,
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

/// Create an account.
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
