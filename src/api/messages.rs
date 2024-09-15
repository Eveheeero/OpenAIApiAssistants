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

#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct MessagesCreate {}
pub(crate) async fn create(
    api_key: impl std::fmt::Display,
    body: MessagesCreate,
) -> Result<super::Message, Error> {
    let response = send(api_key, "".to_string(), None, body).await?;
    reform_data::<Message>(response).await.map(Into::into)
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct MessagesListing {}
pub(crate) async fn listing(
    api_key: impl std::fmt::Display,
    body: MessagesListing,
) -> Result<Vec<super::Message>, Error> {
    let response = send(api_key, "".to_string(), None, body).await?;
    reform_data::<Vec<Message>>(response)
        .await
        .map(|x| x.into_iter().map(Into::into).collect())
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct MessagesGet {}
pub(crate) async fn get(
    api_key: impl std::fmt::Display,
    body: MessagesGet,
) -> Result<super::Message, Error> {
    let response = send(api_key, "".to_string(), None, body).await?;
    reform_data::<Message>(response).await.map(Into::into)
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct MessagesDelete {}
pub(crate) async fn delete(
    api_key: impl std::fmt::Display,
    body: MessagesDelete,
) -> Result<(), Error> {
    let _response = send(api_key, "".to_string(), None, body).await?;
    Ok(())
}
#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct MessagesModify {}
pub(crate) async fn modify(
    api_key: impl std::fmt::Display,
    body: MessagesModify,
) -> Result<super::Message, Error> {
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
impl From<Message> for super::Message {
    fn from(value: Message) -> Self {
        Self {}
    }
}
