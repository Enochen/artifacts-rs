use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`ask_game_assistant`]
#[derive(Clone, Debug)]
pub struct AskGameAssistantParams {
    pub assistant_question_schema: models::AssistantQuestionSchema,
}

impl AskGameAssistantParams {
    pub fn new(assistant_question_schema: models::AssistantQuestionSchema) -> Self {
        Self {
            assistant_question_schema,
        }
    }
}

/// struct for typed errors of method [`ask_game_assistant`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AskGameAssistantError {
    /// Access denied, you must be a member to do that.
    Status451(models::ErrorResponseSchema),
    /// You have reached the daily limit for assistant questions. Try again tomorrow.
    Status570(models::ErrorResponseSchema),
    /// Insufficient gems.
    Status563(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for AskGameAssistantError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            451 => Ok(Self::Status451(raw)),
            570 => Ok(Self::Status570(raw)),
            563 => Ok(Self::Status563(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// Ask the game assistant a question about game mechanics or public API usage. An active membership is required. Members receive a limited number of free questions per day. When no free question is available, the request can spend 1 gem with pay_with_gems=true.
pub async fn ask_game_assistant(
    configuration: &configuration::Configuration,
    params: AskGameAssistantParams,
) -> Result<models::AssistantAnswerSchema, Error<AskGameAssistantError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let assistant_question_schema = params.assistant_question_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/game_assistant/ask", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&assistant_question_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AskGameAssistantError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
