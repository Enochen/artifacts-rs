use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`fight_simulation`]
#[derive(Clone, Debug)]
pub struct FightSimulationParams {
    pub combat_simulation_request_schema: models::CombatSimulationRequestSchema,
}

impl FightSimulationParams {
    pub fn new(combat_simulation_request_schema: models::CombatSimulationRequestSchema) -> Self {
        Self {
            combat_simulation_request_schema,
        }
    }
}

/// struct for typed errors of method [`fight_simulation`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FightSimulationError {
    /// Monster not found.
    Status404(models::ErrorResponseSchema),
    /// Access denied, you must be a member to do that.
    Status451(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for FightSimulationError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            451 => Ok(Self::Status451(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// Simulate combat with fake characters against a monster multiple times. Member or founder account required.
pub async fn fight_simulation(
    configuration: &configuration::Configuration,
    params: FightSimulationParams,
) -> Result<models::CombatSimulationResponseSchema, Error<FightSimulationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let combat_simulation_request_schema = params.combat_simulation_request_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/simulation/fight_simulation",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&combat_simulation_request_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FightSimulationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
