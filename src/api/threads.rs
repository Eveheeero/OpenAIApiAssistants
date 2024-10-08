use super::reform_data;
use crate::prelude::*;
use std::collections::HashMap;

const ENDPOINT: &str = "https://api.openai.com/v1/threads";

async fn send(
    api_key: impl std::fmt::Display,
    threads_id: Option<String>,
    body: impl serde::Serialize,
) -> Result<reqwest::Response, Error> {
    let endpoint = if let Some(threads_id) = threads_id {
        format!("{}/{}", ENDPOINT, threads_id)
    } else {
        ENDPOINT.to_string()
    };
    reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|_| Error::RequestFailed)
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct ThreadsCreate {}
pub(crate) async fn create(
    api_key: impl std::fmt::Display,
    body: ThreadsCreate,
) -> Result<super::Thread, Error> {
    let response = send(api_key, None, body).await?;
    reform_data::<Thread>(response).await.map(Into::into)
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct ThreadsGet {}
pub(crate) async fn get(
    api_key: impl std::fmt::Display,
    body: ThreadsGet,
) -> Result<super::Thread, Error> {
    let response = send(api_key, None, body).await?;
    reform_data::<Thread>(response).await.map(Into::into)
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct ThreadsDelete {}
pub(crate) async fn delete(
    api_key: impl std::fmt::Display,
    body: ThreadsDelete,
) -> Result<(), Error> {
    let _response = send(api_key, None, body).await?;
    Ok(())
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct ThreadsModify {}
pub(crate) async fn modify(
    api_key: impl std::fmt::Display,
    body: ThreadsModify,
) -> Result<super::Thread, Error> {
    let response = send(api_key, None, body).await?;
    reform_data::<Thread>(response).await.map(Into::into)
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Thread {
    id: String,
    object: String,
    created_at: u32,
    tool_resources: Option<HashMap<String, Vec<String>>>,
    metadata: HashMap<String, String>,
}
impl From<Thread> for super::Thread {
    fn from(value: Thread) -> Self {
        Self {}
    }
}
