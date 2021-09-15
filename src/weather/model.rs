use serde::Deserialize;

use super::emoji::get_emoji;

#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
}

#[derive(Debug, Deserialize)]
struct WeatherMap {
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    main: WeatherMain,
    weather: Vec<WeatherMap>,
}

#[derive(Debug)]
pub struct Weather {
    pub temperature: f32,
    pub icon: String,
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

#[cfg(test)]
mod tests {
    use super::{Weather, WeatherResponse};

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
