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
