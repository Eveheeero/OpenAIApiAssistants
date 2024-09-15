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

pub(crate) async fn create(api_key: impl std::fmt::Display) -> Result<super::Assistant, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, None, body).await?;
    reform_data::<Assistant>(response).await.map(Into::into)
}
pub(crate) async fn list(api_key: impl std::fmt::Display) -> Result<Vec<super::Assistant>, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, None, body).await?;
    reform_data::<Vec<Assistant>>(response)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())
}
pub(crate) async fn get(api_key: impl std::fmt::Display) -> Result<super::Assistant, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, None, body).await?;
    reform_data::<Assistant>(response).await.map(Into::into)
}
pub(crate) async fn delete(api_key: impl std::fmt::Display) -> Result<(), Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let _response = send(api_key, None, body).await?;
    Ok(())
}
pub(crate) async fn modify(api_key: impl std::fmt::Display) -> Result<super::Assistant, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
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
