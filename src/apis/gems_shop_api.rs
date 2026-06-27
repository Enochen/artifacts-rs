use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`buy_custom_design`]
#[derive(Clone, Debug)]
pub struct BuyCustomDesignParams {
    pub buy_custom_design_request_schema: models::BuyCustomDesignRequestSchema,
}

impl BuyCustomDesignParams {
    pub fn new(buy_custom_design_request_schema: models::BuyCustomDesignRequestSchema) -> Self {
        Self {
            buy_custom_design_request_schema,
        }
    }
}

/// struct for passing parameters to the method [`buy_skin`]
#[derive(Clone, Debug)]
pub struct BuySkinParams {
    pub buy_skin_request_schema: models::BuySkinRequestSchema,
}

impl BuySkinParams {
    pub fn new(buy_skin_request_schema: models::BuySkinRequestSchema) -> Self {
        Self {
            buy_skin_request_schema,
        }
    }
}

/// struct for passing parameters to the method [`buy_spawn_event`]
#[derive(Clone, Debug)]
pub struct BuySpawnEventParams {
    pub spawn_event_request_schema: models::SpawnEventRequestSchema,
}

impl BuySpawnEventParams {
    pub fn new(spawn_event_request_schema: models::SpawnEventRequestSchema) -> Self {
        Self {
            spawn_event_request_schema,
        }
    }
}

/// struct for typed errors of method [`buy_custom_design`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BuyCustomDesignError {
    /// Insufficient gems.
    Status563(models::ErrorResponseSchema),
    /// Custom design not found.
    Status404(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for BuyCustomDesignError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            563 => Ok(Self::Status563(raw)),
            404 => Ok(Self::Status404(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`buy_skin`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BuySkinError {
    /// Skin not found.
    Status404(models::ErrorResponseSchema),
    /// You already own this skin.
    Status551(models::ErrorResponseSchema),
    /// This skin is not available for purchase.
    Status552(models::ErrorResponseSchema),
    /// Insufficient gems.
    Status563(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for BuySkinError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            404 => Ok(Self::Status404(raw)),
            551 => Ok(Self::Status551(raw)),
            552 => Ok(Self::Status552(raw)),
            563 => Ok(Self::Status563(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`buy_spawn_event`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BuySpawnEventError {
    /// Insufficient gems.
    Status563(models::ErrorResponseSchema),
    /// Event not found.
    Status404(models::ErrorResponseSchema),
    /// Event already active or maximum active events reached.
    Status564(models::ErrorResponseSchema),
    /// This event is on cooldown and cannot be spawned yet.
    Status566(models::ErrorResponseSchema),
    /// This event is not available for purchase.
    Status571(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for BuySpawnEventError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            563 => Ok(Self::Status563(raw)),
            404 => Ok(Self::Status404(raw)),
            564 => Ok(Self::Status564(raw)),
            566 => Ok(Self::Status566(raw)),
            571 => Ok(Self::Status571(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`buy_subscription`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BuySubscriptionError {
    /// Insufficient gems.
    Status563(models::ErrorResponseSchema),
    /// An active Stripe subscription cannot be extended with gems or member tokens.
    Status573(models::ErrorResponseSchema),
    /// Request could not be processed due to an invalid payload.
    Status422(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for BuySubscriptionError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = models::ErrorResponseSchema::deserialize(deserializer)?;
        match raw.error.code {
            563 => Ok(Self::Status563(raw)),
            573 => Ok(Self::Status573(raw)),
            422 => Ok(Self::Status422(raw)),
            _ => Err(de::Error::custom(format!(
                "Unexpected error code: {}",
                raw.error.code
            ))),
        }
    }
}

/// struct for typed errors of method [`get_catalog`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetCatalogError {}

impl<'de> Deserialize<'de> for GetCatalogError {
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

/// Buy a custom design using gems.
pub async fn buy_custom_design(
    configuration: &configuration::Configuration,
    params: BuyCustomDesignParams,
) -> Result<models::GemShopCustomDesignPurchaseResponseSchema, Error<BuyCustomDesignError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let buy_custom_design_request_schema = params.buy_custom_design_request_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/gems_shop/buy_custom_design",
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
    local_var_req_builder = local_var_req_builder.json(&buy_custom_design_request_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BuyCustomDesignError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Buy a skin from the gems shop using gems.
pub async fn buy_skin(
    configuration: &configuration::Configuration,
    params: BuySkinParams,
) -> Result<models::BuySkinResponseSchema, Error<BuySkinError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let buy_skin_request_schema = params.buy_skin_request_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/gems_shop/skin", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&buy_skin_request_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BuySkinError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Spawn an event from the gems shop using gems.
pub async fn buy_spawn_event(
    configuration: &configuration::Configuration,
    params: BuySpawnEventParams,
) -> Result<models::ActiveEventResponseSchema, Error<BuySpawnEventError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let spawn_event_request_schema = params.spawn_event_request_schema;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/gems_shop/spawn_event",
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
    local_var_req_builder = local_var_req_builder.json(&spawn_event_request_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BuySpawnEventError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Buy or extend membership by 30 days using gems. Unavailable while a Stripe subscription is active.
pub async fn buy_subscription(
    configuration: &configuration::Configuration,
) -> Result<models::GemShopSubscriptionResponseSchema, Error<BuySubscriptionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/gems_shop/subscription",
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BuySubscriptionError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Return the gems shop catalog.
pub async fn get_catalog(
    configuration: &configuration::Configuration,
) -> Result<models::GemShopCatalogResponseSchema, Error<GetCatalogError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/gems_shop/", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetCatalogError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
