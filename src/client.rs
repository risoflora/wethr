use std::{result, time::Duration};

use reqwest::{Client as HttpClient, ClientBuilder as HttpClientBuilder, Error as ReqError};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::consts::{PROGRAM_NAME, PROGRAM_VERSION};

pub static CLIENT_CONNECT_TIMEOUT: u64 = 5;

pub static CLIENT_TIMEOUT: u64 = 30;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    Reqwest(#[from] ReqError),
}

pub type ClientResult<T> = result::Result<T, ClientError>;

#[derive(Clone, Debug)]
pub struct Client {
    inner: HttpClient,
}

impl Client {
    pub async fn execute<T: DeserializeOwned>(&self, url: &str) -> ClientResult<T> {
        let res = self.inner.get(url).send().await?;
        Ok(res.json::<T>().await?)
    }
}

#[derive(Debug)]
pub struct ClientBuilder {
    inner: HttpClientBuilder,
}

pub type ClientBuilderResult = result::Result<Client, ClientError>;

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            inner: HttpClientBuilder::new()
                .gzip(true)
                .user_agent(format!("{} {}", PROGRAM_NAME, PROGRAM_VERSION))
                .timeout(Duration::from_secs(CLIENT_TIMEOUT))
                .connect_timeout(Duration::from_secs(CLIENT_CONNECT_TIMEOUT)),
        }
    }

    pub fn set_connect_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.connect_timeout(timeout))
    }

    pub fn set_timeout(self, timeout: Duration) -> Self {
        self.with_inner(|inner| inner.timeout(timeout))
    }

    pub fn build(self) -> ClientBuilderResult {
        Ok(Client {
            inner: self.inner.build()?,
        })
    }

    #[inline]
    fn with_inner<F>(mut self, func: F) -> Self
    where
        F: FnOnce(HttpClientBuilder) -> HttpClientBuilder,
    {
        self.inner = func(self.inner);
        self
    }
}
