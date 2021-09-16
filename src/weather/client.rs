use std::time::Duration;

use serde::Deserialize;
use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    datetime::DateTime,
    emoji::get_emoji,
    location::model::Coordinates,
    units::Units,
    weather::model::{Weather, Wind},
};

pub const URL: &str = "http://api.openweathermap.org/data/2.5/weather";

pub const TOKEN: &str = "315bfb21a64943c67a92e2da0022fdbe";

#[derive(Error, Debug)]
pub enum WeatherClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
}

#[derive(Debug)]
pub struct WeatherClient {
    inner: ClientBuilder,
}

#[derive(Debug, Deserialize)]
struct WeatherMap {
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: i32,
    sea_level: i32,
    grnd_level: i32,
}

#[derive(Debug, Deserialize)]
struct WeatherWindMap {
    speed: f32,
    deg: i32,
    gust: f32,
}

impl From<WeatherWindMap> for Wind {
    fn from(response: WeatherWindMap) -> Self {
        Wind {
            speed: response.speed,
            degrees: response.deg,
            gust: response.gust,
        }
    }
}

#[derive(Debug, Deserialize)]
struct WeatherClouds {
    all: i32,
}

#[derive(Debug, Deserialize)]
struct WeatherSys {
    sunrise: u64,
    sunset: u64,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    weather: Vec<WeatherMap>,
    main: WeatherMain,
    wind: WeatherWindMap,
    clouds: WeatherClouds,
    dt: u64,
    sys: WeatherSys,
}

impl From<WeatherResponse> for Weather {
    fn from(response: WeatherResponse) -> Self {
        let weather = &response.weather[0];
        Self {
            temperature: response.main.temp,
            icon: get_emoji(&weather.description)
                .unwrap_or_default()
                .to_string(),
            description: weather.description.clone(),
            feels_like: response.main.feels_like,
            min_temperature: response.main.temp_min,
            max_temperature: response.main.temp_max,
            pressure: response.main.pressure,
            humidity: response.main.humidity,
            sea_level: response.main.sea_level,
            ground_level: response.main.grnd_level,
            wind: response.wind.into(),
            clouds: response.clouds.all,
            date_time: DateTime::from_unix(response.dt),
            sunrise: DateTime::from_unix(response.sys.sunrise),
            sunset: DateTime::from_unix(response.sys.sunset),
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
              \"temp\": 25.8,
              \"feels_like\": 25.87,
              \"temp_min\": 25.8,
              \"temp_max\": 25.8,
              \"pressure\": 1017,
              \"humidity\": 55,
              \"sea_level\": 1017,
              \"grnd_level\": 949
            },
            \"wind\": { \"speed\": 4.72, \"deg\": 115, \"gust\": 6.14 },
            \"clouds\": { \"all\": 46 },
            \"dt\": 1631620646,
            \"sys\": { \"country\": \"BR\", \"sunrise\": 1631607769, \"sunset\": 1631651152 }
        }";
        let response = serde_json::from_str::<WeatherResponse>(json);
        assert!(response.is_ok());
        let weather: Weather = response.unwrap().into();
        assert_eq!(weather.temperature, 25.8);
        assert_eq!(weather.icon, "☁️");
        assert_eq!(weather.description, "scattered clouds");
        assert_eq!(weather.feels_like, 25.87);
        assert_eq!(weather.min_temperature, 25.8);
        assert_eq!(weather.max_temperature, 25.8);
        assert_eq!(weather.pressure, 1017);
        assert_eq!(weather.humidity, 55);
        assert_eq!(weather.sea_level, 1017);
        assert_eq!(weather.ground_level, 949);
        assert_eq!(weather.wind.speed, 4.72);
        assert_eq!(weather.wind.degrees, 115);
        assert_eq!(weather.wind.gust, 6.14);
        assert_eq!(weather.clouds, 46);
        assert_eq!(weather.date_time.to_string(), "2021-09-14T11:57:26Z");
        assert_eq!(weather.sunrise.to_string(), "2021-09-14T08:22:49Z");
        assert_eq!(weather.sunset.to_string(), "2021-09-14T20:25:52Z");
    }
}
