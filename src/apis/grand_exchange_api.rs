use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_ge_history`]
#[derive(Clone, Debug)]
pub struct GetGeHistoryParams {
    /// The code of the item.
    pub code: String,
    /// Account involved in the transaction (matches either seller or buyer).
    pub account: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetGeHistoryParams {
    pub fn new(
        code: String,
        account: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            code,
            account,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_ge_order`]
#[derive(Clone, Debug)]
pub struct GetGeOrderParams {
    /// The id of the order.
    pub id: String,
}

impl GetGeOrderParams {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// struct for passing parameters to the method [`get_ge_orders`]
#[derive(Clone, Debug)]
pub struct GetGeOrdersParams {
    /// The code of the item.
    pub code: Option<String>,
    /// The account that sells or buys items.
    pub account: Option<String>,
    /// Filter by order type (sell or buy).
    pub r#type: Option<models::GeOrderType>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetGeOrdersParams {
    pub fn new(
        code: Option<String>,
        account: Option<String>,
        r#type: Option<models::GeOrderType>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            code,
            account,
            r#type,
            page,
            size,
        }
    }
}

/// struct for typed errors of method [`get_ge_history`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetGeHistoryError {
    /// item history not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetGeHistoryError {
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

/// struct for typed errors of method [`get_ge_order`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetGeOrderError {
    /// GE order not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetGeOrderError {
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

/// struct for typed errors of method [`get_ge_orders`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetGeOrdersError {}

impl<'de> Deserialize<'de> for GetGeOrdersError {
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

/// Fetch the transaction history of the item for the last 7 days (buy and sell orders).
pub async fn get_ge_history(
    configuration: &configuration::Configuration,
    params: GetGeHistoryParams,
) -> Result<models::DataPageGeOrderHistorySchema, Error<GetGeHistoryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let account = params.account;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/grandexchange/history/{code}",
        local_var_configuration.base_path,
        code = crate::apis::urlencode(code)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = account {
        local_var_req_builder =
            local_var_req_builder.query(&[("account", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetGeHistoryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a specific order by ID.
pub async fn get_ge_order(
    configuration: &configuration::Configuration,
    params: GetGeOrderParams,
) -> Result<models::GeOrderResponseSchema, Error<GetGeOrderError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/grandexchange/orders/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
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
        let local_var_entity: Option<GetGeOrderError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch all orders (sell and buy orders).  Use the `type` parameter to filter by order type; when using `account`, `type` is required to decide whether to match seller or buyer.
pub async fn get_ge_orders(
    configuration: &configuration::Configuration,
    params: GetGeOrdersParams,
) -> Result<models::DataPageGeOrderSchema, Error<GetGeOrdersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let account = params.account;
    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/grandexchange/orders", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = code {
        local_var_req_builder =
            local_var_req_builder.query(&[("code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = account {
        local_var_req_builder =
            local_var_req_builder.query(&[("account", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetGeOrdersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
