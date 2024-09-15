use super::reform_data;
use crate::prelude::*;
use std::collections::HashMap;

async fn send(
    api_key: impl std::fmt::Display,
    thread_id: String,
    message_id: Option<String>,
    body: impl serde::Serialize,
) -> Result<reqwest::Response, Error> {
    let endpoint = if message_id.is_some() {
        let message_id = message_id.unwrap();
        format!("https://api.openai.com/v1/threads/{thread_id}/messages/{message_id}")
    } else {
        format!("https://api.openai.com/v1/threads/{thread_id}/messages")
    };
    reqwest::Client::new()
        .post(endpoint)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|_| Error::RequestFailed)
}

pub(crate) async fn create(api_key: impl std::fmt::Display) -> Result<super::Message, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    reform_data::<Message>(response).await.map(Into::into)
}
pub(crate) async fn list(api_key: impl std::fmt::Display) -> Result<Vec<super::Message>, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    reform_data::<Vec<Message>>(response)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())
}
pub(crate) async fn get(api_key: impl std::fmt::Display) -> Result<super::Message, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    reform_data::<Message>(response).await.map(Into::into)
}
pub(crate) async fn delete(api_key: impl std::fmt::Display) -> Result<(), Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let _response = send(api_key, "".to_string(), None, body).await?;
    Ok(())
}
pub(crate) async fn modify(api_key: impl std::fmt::Display) -> Result<super::Message, Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    reform_data::<Message>(response).await.map(Into::into)
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct Message {
    id: String,
    object: String,
    created_at: u32,
    thread_id: String,
    status: String,
    incomplete_details: Option<HashMap<String, String>>,
    completed_at: Option<u32>,
    incomplete_ut: Option<u32>,
    role: String,
    content: Vec<()>,
    assistant_id: Option<String>,
    run_id: Option<String>,
    attachments: Option<Vec<serde_json::Value>>,
    metadata: HashMap<String, String>,
}
