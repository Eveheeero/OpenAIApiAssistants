use crate::prelude::*;

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

pub(crate) async fn create(api_key: impl std::fmt::Display) -> Result<(), Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    Ok(())
}
pub(crate) async fn list(api_key: impl std::fmt::Display) -> Result<(), Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    Ok(())
}
pub(crate) async fn get(api_key: impl std::fmt::Display) -> Result<(), Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    Ok(())
}
pub(crate) async fn delete(api_key: impl std::fmt::Display) -> Result<(), Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    Ok(())
}
pub(crate) async fn modify(api_key: impl std::fmt::Display) -> Result<(), Error> {
    #[derive(Debug, serde::Serialize)]
    struct Body {}

    let body = Body {};
    let response = send(api_key, "".to_string(), None, body).await?;
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Message {}
