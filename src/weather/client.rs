use std::time::Duration;

use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    location::model::Coordinates,
    units::Units,
    weather::model::{Weather, WeatherResponse},
};

pub const URL: &str = "http://api.openweathermap.org/data/2.5/weather";

pub const TOKEN: &str = "315bfb21a64943c67a92e2da0022fdbe";

#[derive(Error, Debug)]
pub enum WeatherClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
}

pub struct WeatherClient {
    inner: ClientBuilder,
}

impl WeatherClient {
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

    pub async fn get_with_units(
        self,
        coordinates: &Coordinates,
        units: Units,
    ) -> Result<Weather, WeatherClientError> {
        let res: WeatherResponse = self
            .inner
            .build()?
            .get(&format!(
                "{}?lat={lat}&lon={lon}&appid={appid}&units={units}",
                URL,
                lat = coordinates.latitude,
                lon = coordinates.longitude,
                appid = TOKEN.to_string(),
                units = units.to_string()
            ))
            .await?;
        Ok(res.into())
    }

    pub async fn get(self, coordinates: &Coordinates) -> Result<Weather, WeatherClientError> {
        self.get_with_units(coordinates, Units::default()).await
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
    use super::WeatherClient;
    use crate::{location::model::Coordinates, units::Units};

    #[tokio::test]
    async fn client_get_with_units() {
        let coordinates = Coordinates {
            latitude: -7.9194,
            longitude: -37.175,
        };
        assert!(WeatherClient::new()
            .get_with_units(&coordinates, Units::Fahrenheit)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn client_get() {
        let coordinates = Coordinates {
            latitude: -7.9194,
            longitude: -37.175,
        };
        assert!(WeatherClient::new().get(&coordinates).await.is_ok());
    }
}
