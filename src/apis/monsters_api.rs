use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_all_monsters`]
#[derive(Clone, Debug)]
pub struct GetAllMonstersParams {
    /// Name of the monster.
    pub name: Option<String>,
    /// Monster minimum level.
    pub min_level: Option<u32>,
    /// Monster maximum level.
    pub max_level: Option<u32>,
    /// Item code of the drop.
    pub drop: Option<String>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

impl GetAllMonstersParams {
    pub fn new(
        name: Option<String>,
        min_level: Option<u32>,
        max_level: Option<u32>,
        drop: Option<String>,
        page: Option<u32>,
        size: Option<u32>,
    ) -> Self {
        Self {
            name,
            min_level,
            max_level,
            drop,
            page,
            size,
        }
    }
}

/// struct for passing parameters to the method [`get_monster`]
#[derive(Clone, Debug)]
pub struct GetMonsterParams {
    /// The code of the monster.
    pub code: String,
}

impl GetMonsterParams {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// struct for typed errors of method [`get_all_monsters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllMonstersError {}

impl TryFrom<StatusCode> for GetAllMonstersError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_monster`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMonsterError {
    /// Monster not found.
    Status404,
}

impl TryFrom<StatusCode> for GetMonsterError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// Fetch monsters details.
pub async fn get_all_monsters(
    configuration: &configuration::Configuration,
    params: GetAllMonstersParams,
) -> Result<models::DataPageMonsterSchema, Error<GetAllMonstersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    // unbox the parameters
    let min_level = params.min_level;
    // unbox the parameters
    let max_level = params.max_level;
    // unbox the parameters
    let drop = params.drop;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/monsters", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = name {
        local_var_req_builder =
            local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = min_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("min_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("max_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = drop {
        local_var_req_builder =
            local_var_req_builder.query(&[("drop", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllMonstersError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a monster.
pub async fn get_monster(
    configuration: &configuration::Configuration,
    params: GetMonsterParams,
) -> Result<models::MonsterResponseSchema, Error<GetMonsterError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/monsters/{code}",
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
        let local_var_entity: Option<GetMonsterError> = local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
