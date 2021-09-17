use std::time::Duration;

use serde::Deserialize;
use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    location::model::{Coordinates, Location},
};

pub const URL_LOCATION: &str = "https://ipapi.co/json/";

#[derive(Error, Debug)]
pub enum LocationClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
}

#[derive(Debug)]
pub struct LocationClient {
    inner: ClientBuilder,
}

#[derive(Clone, Debug, Deserialize)]
struct LocationResponse {
    pub country: String,
    pub city: String,
    pub latitude: f32,
    pub longitude: f32,
}

impl From<LocationResponse> for Location {
    fn from(response: LocationResponse) -> Self {
        Self {
            country: response.country,
            city: response.city,
            coordinates: Coordinates {
                latitude: response.latitude,
                longitude: response.longitude,
            },
        }
    }
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
        let res: LocationResponse = self.inner.build()?.get(URL_LOCATION).await?;
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
    use super::{Location, LocationClient, LocationResponse};

    #[tokio::test]
    async fn client_get() {
        assert!(LocationClient::new().get().await.is_ok());
    }

    #[test]
    fn location_from_response() {
        let json = "{
                \"country\": \"Brazil\",
                \"city\": \"Monteiro\",
                \"latitude\": -7.9194,
                \"longitude\": -37.175
            }";
        let response = serde_json::from_str::<LocationResponse>(json);
        assert!(response.is_ok());
        let location: Location = response.unwrap().into();
        assert_eq!(location.country, "Brazil");
        assert_eq!(location.city, "Monteiro");
        assert_eq!(location.coordinates.latitude, -7.9194);
        assert_eq!(location.coordinates.longitude, -37.175);
    }
}
