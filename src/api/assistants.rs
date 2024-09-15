use super::reform_data;
use crate::prelude::*;
use std::collections::HashMap;

const ENDPOINT: &str = "https://api.openai.com/v1/assistants";

async fn send(
    api_key: impl std::fmt::Display,
    assistant_id: Option<String>,
    body: impl serde::Serialize,
) -> Result<reqwest::Response, Error> {
    let endpoint = if let Some(assistant_id) = assistant_id {
        format!("{}/{}", ENDPOINT, assistant_id)
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
pub(crate) struct AssistantsCreate {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) model: String,
    pub(crate) instructions: Option<String>,
    pub(crate) tools: Vec<()>,
    pub(crate) tool_resources: Option<HashMap<String, Vec<String>>>,
    pub(crate) metadata: HashMap<String, String>,
    pub(crate) temperature: Option<f32>,
    pub(crate) top_p: Option<f32>,
}
pub(crate) async fn create(
    api_key: impl std::fmt::Display,
    body: AssistantsCreate,
) -> Result<super::Assistant, Error> {
    let response = send(api_key, None, body).await?;
    reform_data::<Assistant>(response).await.map(Into::into)
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct AssistantsListing {}
pub(crate) async fn listing(
    api_key: impl std::fmt::Display,
    body: AssistantsListing,
) -> Result<Vec<super::Assistant>, Error> {
    let response = send(api_key, None, body).await?;
    reform_data::<Vec<Assistant>>(response)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct AssistantsGet {}
pub(crate) async fn get(
    api_key: impl std::fmt::Display,
    body: AssistantsGet,
) -> Result<super::Assistant, Error> {
    let response = send(api_key, None, body).await?;
    reform_data::<Assistant>(response).await.map(Into::into)
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct AssistantsDelete {}
pub(crate) async fn delete(
    api_key: impl std::fmt::Display,
    body: AssistantsDelete,
) -> Result<(), Error> {
    let _response = send(api_key, None, body).await?;
    Ok(())
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct AssistantsModify {}
pub(crate) async fn modify(
    api_key: impl std::fmt::Display,
    body: AssistantsModify,
) -> Result<super::Assistant, Error> {
    let response = send(api_key, None, body).await?;
    reform_data::<Assistant>(response).await.map(Into::into)
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Assistant {
    id: String,
    object: String,
    created_at: u32,
    name: Option<String>,
    description: Option<String>,
    model: String,
    instructions: Option<String>,
    tools: Vec<()>,
    tool_resources: Option<HashMap<String, Vec<String>>>,
    metadata: HashMap<String, String>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    response_format: String,
}
impl From<Assistant> for super::Assistant {
    fn from(value: Assistant) -> Self {
        Self {}
    }
}
