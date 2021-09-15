use std::time::Duration;

use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    location::model::Location,
};

use super::model::LocationResponse;

pub const URL: &str = "http://ip-api.com/json";

#[derive(Error, Debug)]
pub enum LocationClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
}

pub struct LocationClient {
    inner: ClientBuilder,
}

impl LocationClient {
    pub fn new() -> Self {
        Self {
            inner: ClientBuilder::new(),
        }
    }

    pub fn set_connect_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.set_connect_timeout(timeout))
    }

    pub fn set_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.set_timeout(timeout))
    }

    pub async fn get(self) -> Result<Location, LocationClientError> {
        let res: LocationResponse = self.inner.build()?.get(URL).await?;
        Ok(res.into())
    }

    #[inline]
    fn with_inner<F>(mut self, func: F) -> Self
    where
        F: FnOnce(ClientBuilder) -> ClientBuilder,
    {
        self.inner = func(self.inner);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::LocationClient;

    #[tokio::test]
    async fn client_get() {
        assert!(LocationClient::new().get().await.is_ok());
    }
}
