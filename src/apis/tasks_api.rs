use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`get_all_tasks_rewards_tasks_rewards_get`]
#[derive(Clone, Debug)]
pub struct GetAllTasksRewardsTasksRewardsGetParams {
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_all_tasks_tasks_list_get`]
#[derive(Clone, Debug)]
pub struct GetAllTasksTasksListGetParams {
    /// Minimum level.
    pub min_level: Option<u32>,
    /// Maximum level.
    pub max_level: Option<u32>,
    /// The code of the skill.
    pub skill: Option<models::Skill>,
    /// The type of tasks.
    pub r#type: Option<models::TaskType>,
    /// Page number
    pub page: Option<u32>,
    /// Page size
    pub size: Option<u32>,
}

/// struct for passing parameters to the method [`get_task_tasks_list_code_get`]
#[derive(Clone, Debug)]
pub struct GetTaskTasksListCodeGetParams {
    /// The code of the task.
    pub code: String,
}

/// struct for passing parameters to the method [`get_tasks_reward_tasks_rewards_code_get`]
#[derive(Clone, Debug)]
pub struct GetTasksRewardTasksRewardsCodeGetParams {
    /// The code of the tasks reward.
    pub code: String,
}

/// struct for typed errors of method [`get_all_tasks_rewards_tasks_rewards_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllTasksRewardsTasksRewardsGetError {}

impl TryFrom<StatusCode> for GetAllTasksRewardsTasksRewardsGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_all_tasks_tasks_list_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllTasksTasksListGetError {}

impl TryFrom<StatusCode> for GetAllTasksTasksListGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_task_tasks_list_code_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTaskTasksListCodeGetError {
    /// Task not found.
    Status404,
}

impl TryFrom<StatusCode> for GetTaskTasksListCodeGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// struct for typed errors of method [`get_tasks_reward_tasks_rewards_code_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTasksRewardTasksRewardsCodeGetError {
    /// Tasks reward not found.
    Status404,
}

impl TryFrom<StatusCode> for GetTasksRewardTasksRewardsCodeGetError {
    type Error = &'static str;
    #[allow(clippy::match_single_binding)]
    fn try_from(status: StatusCode) -> Result<Self, Self::Error> {
        match status.as_u16() {
            404 => Ok(Self::Status404),
            _ => Err("status code not in spec"),
        }
    }
}

/// Fetch the list of all tasks rewards. To obtain these rewards, you must exchange 6 task coins with a tasks master.
pub async fn get_all_tasks_rewards_tasks_rewards_get(
    configuration: &configuration::Configuration,
    params: GetAllTasksRewardsTasksRewardsGetParams,
) -> Result<models::DataPageDropRateSchema, Error<GetAllTasksRewardsTasksRewardsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks/rewards", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetAllTasksRewardsTasksRewardsGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch the list of all tasks.
pub async fn get_all_tasks_tasks_list_get(
    configuration: &configuration::Configuration,
    params: GetAllTasksTasksListGetParams,
) -> Result<models::DataPageTaskFullSchema, Error<GetAllTasksTasksListGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let min_level = params.min_level;
    // unbox the parameters
    let max_level = params.max_level;
    // unbox the parameters
    let skill = params.skill;
    // unbox the parameters
    let r#type = params.r#type;
    // unbox the parameters
    let page = params.page;
    // unbox the parameters
    let size = params.size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks/list", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = min_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("min_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = max_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("max_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = skill {
        local_var_req_builder =
            local_var_req_builder.query(&[("skill", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllTasksTasksListGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a task.
pub async fn get_task_tasks_list_code_get(
    configuration: &configuration::Configuration,
    params: GetTaskTasksListCodeGetParams,
) -> Result<models::TaskFullResponseSchema, Error<GetTaskTasksListCodeGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/tasks/list/{code}",
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
        let local_var_entity: Option<GetTaskTasksListCodeGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the details of a tasks reward.
pub async fn get_tasks_reward_tasks_rewards_code_get(
    configuration: &configuration::Configuration,
    params: GetTasksRewardTasksRewardsCodeGetParams,
) -> Result<models::RewardResponseSchema, Error<GetTasksRewardTasksRewardsCodeGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let code = params.code;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/tasks/rewards/{code}",
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
        let local_var_entity: Option<GetTasksRewardTasksRewardsCodeGetError> =
            local_var_status.try_into().ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
