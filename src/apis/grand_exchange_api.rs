use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_ge_sell_history_grandexchange_history_code_get`]
#[derive(Clone, Debug)]
pub struct GetGeSellHistoryGrandexchangeHistoryCodeGetParams {
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

/// struct for passing parameters to the method [`get_ge_sell_order_grandexchange_orders_id_get`]
#[derive(Clone, Debug)]
pub struct GetGeSellOrderGrandexchangeOrdersIdGetParams {
    /// The id of the order.
    pub id: String,
}

/// struct for passing parameters to the method [`get_ge_sell_orders_grandexchange_orders_get`]
#[derive(Clone, Debug)]
pub struct GetGeSellOrdersGrandexchangeOrdersGetParams {
    /// The code of the item.
    pub code: Option<String>,
    /// The seller (account name) of the item.
    pub seller: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for typed errors of method [`get_ge_sell_history_grandexchange_history_code_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGeSellHistoryGrandexchangeHistoryCodeGetError {
    /// Item not found.
    Status404,
}

impl TryFrom<StatusCode> for GetGeSellHistoryGrandexchangeHistoryCodeGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_ge_sell_order_grandexchange_orders_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGeSellOrderGrandexchangeOrdersIdGetError {
    /// Order not found.
    Status404,
}

impl TryFrom<StatusCode> for GetGeSellOrderGrandexchangeOrdersIdGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_ge_sell_orders_grandexchange_orders_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGeSellOrdersGrandexchangeOrdersGetError {}

impl TryFrom<StatusCode> for GetGeSellOrdersGrandexchangeOrdersGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// Fetch the sales history of the item for the last 7 days.
pub async fn get_ge_sell_history_grandexchange_history_code_get(
    configuration: &configuration::Configuration,
    params: GetGeSellHistoryGrandexchangeHistoryCodeGetParams,
) -> Result<
    models::DataPageGeOrderHistorySchema,
    Error<GetGeSellHistoryGrandexchangeHistoryCodeGetError>,
> {
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
        let local_var_entity: Option<GetGeSellHistoryGrandexchangeHistoryCodeGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the sell order of a item.
pub async fn get_ge_sell_order_grandexchange_orders_id_get(
    configuration: &configuration::Configuration,
    params: GetGeSellOrderGrandexchangeOrdersIdGetParams,
) -> Result<models::GeOrderReponseSchema, Error<GetGeSellOrderGrandexchangeOrdersIdGetError>> {
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
        let local_var_entity: Option<GetGeSellOrderGrandexchangeOrdersIdGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch all sell orders.
pub async fn get_ge_sell_orders_grandexchange_orders_get(
    configuration: &configuration::Configuration,
    params: GetGeSellOrdersGrandexchangeOrdersGetParams,
) -> Result<models::DataPageGeOrderSchema, Error<GetGeSellOrdersGrandexchangeOrdersGetError>> {
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
        let local_var_entity: Option<GetGeSellOrdersGrandexchangeOrdersGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
