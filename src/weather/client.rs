use std::time::Duration;

use serde::Deserialize;
use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    emoji::get_emoji,
    location::model::Coordinates,
    units::Units,
    weather::model::Weather,
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

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
}

#[derive(Debug, Deserialize)]
struct WeatherMap {
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
    weather: Vec<WeatherMap>,
}

impl From<WeatherResponse> for Weather {
    fn from(response: WeatherResponse) -> Self {
        Self {
            temperature: response.main.temp,
            icon: get_emoji(&response.weather[0].description)
                .unwrap_or_default()
                .to_string(),
        }
    }
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
    use super::{Weather, WeatherClient, WeatherResponse};
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

    #[test]
    fn weather_from_response() {
        let json = "{
            \"weather\": [
              {
                \"description\": \"scattered clouds\"
              }
            ],
            \"main\": {
              \"temp\": 25.8
            }
        }";
        let response = serde_json::from_str::<WeatherResponse>(json);
        assert!(response.is_ok());
        let weather: Weather = response.unwrap().into();
        assert_eq!(weather.temperature, 25.8);
        assert_eq!(weather.icon, "☁️");
    }
}
