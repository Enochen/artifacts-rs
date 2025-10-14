use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{de, Deserialize, Deserializer, Serialize};

/// struct for passing parameters to the method [`get_ge_sell_history_by_code`]
#[derive(Clone, Debug)]
pub struct GetGeSellHistoryByCodeParams {
    /// The code of the item.
    pub code: String,
    /// The seller (account name) of the item.
    pub seller: Option<String>,
    /// The buyer (account name) of the item.
    pub buyer: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetGeSellHistoryByCodeParams {
    pub fn new(
        code: String,
        seller: Option<String>,
        buyer: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            code,
            seller,
            buyer,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_ge_sell_order_by_id`]
#[derive(Clone, Debug)]
pub struct GetGeSellOrderByIdParams {
    /// The id of the order.
    pub id: String,
}

impl GetGeSellOrderByIdParams {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// struct for passing parameters to the method [`get_ge_sell_orders`]
#[derive(Clone, Debug)]
pub struct GetGeSellOrdersParams {
    /// The code of the item.
    pub code: Option<String>,
    /// The seller (account name) of the item.
    pub seller: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetGeSellOrdersParams {
    pub fn new(
        code: Option<String>,
        seller: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            code,
            seller,
            page,
            size,
        }
    }
}

/// struct for typed errors of method [`get_ge_sell_history_by_code`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetGeSellHistoryByCodeError {
    /// item history not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetGeSellHistoryByCodeError {
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

/// struct for typed errors of method [`get_ge_sell_order_by_id`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetGeSellOrderByIdError {
    /// GE order not found.
    Status404(models::ErrorResponseSchema),
}

impl<'de> Deserialize<'de> for GetGeSellOrderByIdError {
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

/// struct for typed errors of method [`get_ge_sell_orders`]
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GetGeSellOrdersError {}

impl<'de> Deserialize<'de> for GetGeSellOrdersError {
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

/// Fetch the sales history of the item for the last 7 days.
pub async fn get_ge_sell_history_by_code(
    configuration: &configuration::Configuration,
    params: GetGeSellHistoryByCodeParams,
) -> Result<models::DataPageGeOrderHistorySchema, Error<GetGeSellHistoryByCodeError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let seller = params.seller;
    // unbox the parameters
    let buyer = params.buyer;
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

    if let Some(ref local_var_str) = seller {
        local_var_req_builder =
            local_var_req_builder.query(&[("seller", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = buyer {
        local_var_req_builder =
            local_var_req_builder.query(&[("buyer", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetGeSellHistoryByCodeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the sell order of a item.
pub async fn get_ge_sell_order_by_id(
    configuration: &configuration::Configuration,
    params: GetGeSellOrderByIdParams,
) -> Result<models::GeOrderResponseSchema, Error<GetGeSellOrderByIdError>> {
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
        let local_var_entity: Option<GetGeSellOrderByIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch all sell orders.
pub async fn get_ge_sell_orders(
    configuration: &configuration::Configuration,
    params: GetGeSellOrdersParams,
) -> Result<models::DataPageGeOrderSchema, Error<GetGeSellOrdersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;
    // unbox the parameters
    let seller = params.seller;
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
    if let Some(ref local_var_str) = seller {
        local_var_req_builder =
            local_var_req_builder.query(&[("seller", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetGeSellOrdersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
