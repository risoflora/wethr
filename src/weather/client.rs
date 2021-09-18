use std::time::Duration;

use serde::Deserialize;
use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    consts::TOKEN,
    datetime::DateTime,
    emoji::get_emoji,
    location::model::Coordinates,
    units::Units,
    weather::model::{Weather, Wind},
};

pub const URL_WEATHER: &str = "http://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Error)]
pub enum WeatherClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
}

#[derive(Debug)]
pub struct WeatherClient {
    inner: ClientBuilder,
}

#[derive(Clone, Debug, Deserialize)]
struct WeatherMap {
    description: String,
}

impl Default for WeatherMap {
    fn default() -> Self {
        Self {
            description: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: i32,
    sea_level: Option<i32>,
    grnd_level: Option<i32>,
}

impl Default for WeatherMain {
    fn default() -> Self {
        Self {
            temp: Default::default(),
            feels_like: Default::default(),
            temp_min: Default::default(),
            temp_max: Default::default(),
            pressure: Default::default(),
            humidity: Default::default(),
            sea_level: Default::default(),
            grnd_level: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct WeatherWindMap {
    speed: Option<f32>,
    deg: Option<i32>,
    gust: Option<f32>,
}

impl Default for WeatherWindMap {
    fn default() -> Self {
        Self {
            speed: Default::default(),
            deg: Default::default(),
            gust: Default::default(),
        }
    }
}

impl From<WeatherWindMap> for Wind {
    fn from(response: WeatherWindMap) -> Self {
        Wind {
            speed: response.speed.unwrap_or_default(),
            degrees: response.deg.unwrap_or_default(),
            gust: response.gust.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct WeatherClouds {
    all: i32,
}

impl Default for WeatherClouds {
    fn default() -> Self {
        Self {
            all: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct WeatherSys {
    sunrise: u64,
    sunset: u64,
}

impl Default for WeatherSys {
    fn default() -> Self {
        Self {
            sunrise: Default::default(),
            sunset: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct WeatherResponse {
    weather: Option<Vec<WeatherMap>>,
    main: Option<WeatherMain>,
    wind: Option<WeatherWindMap>,
    clouds: Option<WeatherClouds>,
    dt: Option<u64>,
    sys: Option<WeatherSys>,
}

impl From<WeatherResponse> for Weather {
    fn from(response: WeatherResponse) -> Self {
        let weather = &response.weather.unwrap_or([WeatherMap::default()].to_vec())[0];
        let description = &weather.description;
        let main = response.main.unwrap_or_default();
        let sys = response.sys.unwrap_or_default();
        Self {
            temperature: main.temp,
            icon: get_emoji(&description).unwrap_or_default().to_string(),
            description: format!("{}{}", &description[0..1].to_uppercase(), &description[1..]),
            feels_like: main.feels_like,
            min_temperature: main.temp_min,
            max_temperature: main.temp_max,
            pressure: main.pressure,
            humidity: main.humidity,
            sea_level: main.sea_level,
            ground_level: main.grnd_level,
            wind: response.wind.unwrap_or_default().into(),
            clouds: response.clouds.unwrap_or_default().all,
            date_time: DateTime::from_unix(response.dt.unwrap_or_default()),
            sunrise: DateTime::from_unix(sys.sunrise),
            sunset: DateTime::from_unix(sys.sunset),
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
                "{}?lat={lat}&lon={lon}&units={units}&appid={appid}",
                URL_WEATHER,
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
    use std::time::Duration;

    use tokio::time::sleep;

    use super::{Weather, WeatherClient, WeatherResponse};
    use crate::{location::model::Coordinates, units::Units};

    #[tokio::test]
    async fn client_get_with_units() {
        sleep(Duration::from_secs(1)).await;
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
        sleep(Duration::from_secs(1)).await;
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
        assert_eq!(weather.description, "Scattered clouds");
        assert_eq!(weather.feels_like, 25.87);
        assert_eq!(weather.min_temperature, 25.8);
        assert_eq!(weather.max_temperature, 25.8);
        assert_eq!(weather.pressure, 1017);
        assert_eq!(weather.humidity, 55);
        assert_eq!(weather.sea_level, Some(1017));
        assert_eq!(weather.ground_level, Some(949));
        assert_eq!(weather.wind.speed, 4.72);
        assert_eq!(weather.wind.degrees, 115);
        assert_eq!(weather.wind.gust, 6.14);
        assert_eq!(weather.clouds, 46);
        assert_eq!(weather.date_time.to_string(), "2021-09-14T11:57:26Z");
        assert_eq!(weather.sunrise.to_string(), "2021-09-14T08:22:49Z");
        assert_eq!(weather.sunset.to_string(), "2021-09-14T20:25:52Z");
    }
}
