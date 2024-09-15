pub(crate) mod assistants;
pub(crate) mod messages;
pub(crate) mod threads;
use crate::prelude::*;

async fn reform_data<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, Error> {
    response
        .json::<T>()
        .await
        .map_err(|_| Error::InvalidResponse)
}

#[derive(Debug, Clone)]
pub(crate) struct Message {}
#[derive(Debug, Clone)]
pub(crate) struct Assistant {}
#[derive(Debug, Clone)]
pub(crate) struct Thread {}

impl From<messages::Message> for Message {
    fn from(value: messages::Message) -> Self {
        Self {}
    }
}
impl From<assistants::Assistant> for Assistant {
    fn from(value: assistants::Assistant) -> Self {
        Self {}
    }
}
impl From<threads::Thread> for Thread {
    fn from(value: threads::Thread) -> Self {
        Self {}
    }
}
