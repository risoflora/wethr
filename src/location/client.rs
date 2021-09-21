use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

use serde::Deserialize;
use thiserror::Error;

use crate::{
    client::{ClientBuilder, ClientError},
    consts::TOKEN,
    location::model::{Coordinates, Location},
};

pub const URL_LOCATIONS: [&str; 4] = [
    "http://ip-api.com/json/",
    "https://ipapi.co/json/",
    "https://freegeoip.app/json/",
    "https://ipwhois.app/json/",
];

pub const URL_QUERY_LOCATION: &str = "https://api.openweathermap.org/geo/1.0/direct";

#[derive(Debug, Error)]
pub enum LocationClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
    #[error("Wrong location provider")]
    WrongLocationProvider,
    #[error("Wrong query parameter")]
    WrongQueryParam,
    #[error("Returning {0} cities, please choose one:\n{1}")]
    MoreThanOne(usize, String),
}

#[derive(Debug)]
pub struct LocationClient {
    inner: ClientBuilder,
}

pub type LocationProvider = i8;

#[derive(Clone, Debug, Deserialize)]
struct LocationResponse {
    pub city: String,
    pub country: Option<String>,
    pub country_name: Option<String>,
    #[serde(alias = "lat")]
    pub latitude: f32,
    #[serde(alias = "lon")]
    pub longitude: f32,
}

impl From<LocationResponse> for Location {
    fn from(response: LocationResponse) -> Self {
        Self {
            city: response.city,
            country: response
                .country_name
                .unwrap_or(response.country.unwrap_or("N/D".to_string())),
            coordinates: Coordinates::new(response.latitude, response.longitude),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocationQuery {
    pub city_name: String,
    pub state_code: Option<String>,
    pub country_code: Option<String>,
}

impl From<String> for LocationQuery {
    fn from(query: String) -> Self {
        let values: Vec<_> = query.split(",").collect();
        Self {
            city_name: values[0].to_string(),
            state_code: values.get(1).map(|s| s.to_string()),
            country_code: values.get(2).map(|s| s.to_string()),
        }
    }
}

impl Display for LocationQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.city_name)?;
        if let Some(state_code) = &self.state_code {
            write!(f, ",{}", state_code)?;
        }
        if let Some(country_code) = &self.country_code {
            write!(f, ",{}", country_code)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct LocationQueryResponse {
    pub name: String,
    pub state: Option<String>,
    pub country: Option<String>,
    pub lat: f32,
    pub lon: f32,
}

impl From<LocationQueryResponse> for Location {
    fn from(response: LocationQueryResponse) -> Self {
        Self {
            city: response.name,
            country: response.country.unwrap_or("N/D".to_string()),
            coordinates: Coordinates::new(response.lat, response.lon),
        }
    }
}

impl Default for LocationQueryResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            state: Default::default(),
            country: Default::default(),
            lat: Default::default(),
            lon: Default::default(),
        }
    }
}

impl Display for LocationQueryResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "City: {}\n", &self.name)?;
        if let Some(state_code) = &self.state {
            write!(f, "State code: {}\n", state_code)?;
        }
        if let Some(country_code) = &self.country {
            write!(f, "Country code: {}\n", country_code)?;
        }
        write!(f, "Coordinates:\n")?;
        write!(f, "  Latitude: {}\n", self.lat)?;
        write!(f, "  Longitude: {}", self.lon)
    }
}

#[derive(Clone, Debug)]
pub struct LocationQueryResponses {
    vec: Vec<LocationQueryResponse>,
}

impl LocationQueryResponses {
    fn new(vec: Vec<LocationQueryResponse>) -> Self {
        Self { vec }
    }
}

impl Display for LocationQueryResponses {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for res in &self.vec {
            write!(f, "\n{}\n", res.to_string())?;
        }
        Ok(())
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

    pub async fn get(
        self,
        provider: Option<LocationProvider>,
    ) -> Result<Location, LocationClientError> {
        let provider_number = provider.unwrap_or_default();
        if provider_number < 0 || provider_number as usize >= URL_LOCATIONS.len() {
            return Err(LocationClientError::WrongLocationProvider);
        }
        let res: LocationResponse = self
            .inner
            .build()?
            .get(URL_LOCATIONS[provider_number as usize])
            .await?;
        Ok(res.into())
    }

    pub async fn get_by_query(
        self,
        query: &LocationQuery,
    ) -> Result<Location, LocationClientError> {
        if query.city_name.len() < 2 {
            return Err(LocationClientError::WrongQueryParam);
        };
        let url = format!(
            "{url}?q={query}&limit=5&appid={appid}",
            url = URL_QUERY_LOCATION,
            query = query,
            appid = TOKEN
        );
        let res = LocationQueryResponses::new(self.inner.build()?.get(&url).await?);
        if res.vec.len() > 1 {
            return Err(LocationClientError::MoreThanOne(
                res.vec.len(),
                res.to_string(),
            ));
        }
        let loc = res
            .vec
            .first()
            .unwrap_or(&LocationQueryResponse::default())
            .clone();
        Ok(loc.into())
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

    use super::{
        Location, LocationClient, LocationQuery, LocationQueryResponse, LocationQueryResponses,
        LocationResponse,
    };

    #[tokio::test]
    async fn location_client_get() {
        sleep(Duration::from_secs(1)).await;
        assert!(LocationClient::new().get(None).await.is_ok());
        sleep(Duration::from_secs(1)).await;
        assert!(LocationClient::new().get(Some(-1)).await.is_err());
        sleep(Duration::from_secs(1)).await;
        assert!(LocationClient::new().get(Some(10)).await.is_err());
    }

    #[test]
    fn location_from_response() {
        let json = "{
                \"city\": \"Monteiro\",
                \"country_name\": \"Brazil\",
                \"latitude\": -7.9194,
                \"longitude\": -37.175
            }";
        let response = serde_json::from_str::<LocationResponse>(json);
        assert!(response.is_ok());
        let location: Location = response.unwrap().into();
        assert_eq!(location.city, "Monteiro");
        assert_eq!(location.country, "Brazil");
        assert_eq!(location.coordinates.latitude, -7.9194);
        assert_eq!(location.coordinates.longitude, -37.175);
    }

    #[tokio::test]
    async fn location_client_get_by_query() {
        sleep(Duration::from_secs(1)).await;
        let query = LocationQuery::from("monteiro".to_string());
        assert!(LocationClient::new().get_by_query(&query).await.is_ok());

        let query = LocationQuery::from("".to_string());
        assert!(LocationClient::new().get_by_query(&query).await.is_err());

        let query = LocationQuery::from("london".to_string());
        assert!(LocationClient::new().get_by_query(&query).await.is_err());
    }

    #[test]
    fn location_from_query_response() {
        let json = "{
                \"name\": \"Monteiro\",
                \"state\": \"PB\",
                \"country\": \"BR\",
                \"lat\": -7.9194,
                \"lon\": -37.175
            }";
        let response = serde_json::from_str::<LocationQueryResponse>(json);
        assert!(response.is_ok());
        let location: Location = response.unwrap().into();
        assert_eq!(location.city, "Monteiro");
        assert_eq!(location.country, "BR");
        assert_eq!(location.coordinates.latitude, -7.9194);
        assert_eq!(location.coordinates.longitude, -37.175);
    }

    #[test]
    fn location_query_display() {
        let query = LocationQuery::from("".to_string());
        assert_eq!(query.to_string(), "");
        let query = LocationQuery::from("joão pessoa".to_string());
        assert_eq!(query.to_string(), "joão pessoa");
        let query = LocationQuery::from("joão pessoa,pb".to_string());
        assert_eq!(query.to_string(), "joão pessoa,pb");
        let query = LocationQuery::from("joão pessoa,pb,br".to_string());
        assert_eq!(query.to_string(), "joão pessoa,pb,br");
    }

    #[test]
    fn location_query_response_display() {
        let location = LocationQueryResponse {
            name: "Monteiro".to_string(),
            state: Some("PB".to_string()),
            country: Some("BR".to_string()),
            lat: 12.34,
            lon: 56.78,
        };
        let text = "City: Monteiro
State code: PB
Country code: BR
Coordinates:
  Latitude: 12.34
  Longitude: 56.78";
        assert_eq!(location.to_string(), text);

        let location = LocationQueryResponse {
            name: "Monteiro".to_string(),
            state: None,
            country: Some("BR".to_string()),
            lat: 12.34,
            lon: 56.78,
        };
        let text = "City: Monteiro
Country code: BR
Coordinates:
  Latitude: 12.34
  Longitude: 56.78";
        assert_eq!(location.to_string(), text);

        let location = LocationQueryResponse {
            name: "Monteiro".to_string(),
            state: None,
            country: None,
            lat: 12.34,
            lon: 56.78,
        };
        let text = "City: Monteiro
Coordinates:
  Latitude: 12.34
  Longitude: 56.78";
        assert_eq!(location.to_string(), text);
    }

    #[test]
    fn location_query_responses_display() {
        let json = "[
            {
              \"name\": \"London\",
              \"lat\": 51.5085,
              \"lon\": -0.1257,
              \"country\": \"GB\"
            },
            {
              \"name\": \"London\",
              \"lat\": 42.9834,
              \"lon\": -81.233,
              \"country\": \"CA\"
            },
            {
              \"name\": \"London\",
              \"lat\": 39.8865,
              \"lon\": -83.4483,
              \"country\": \"US\",
              \"state\": \"OH\"
            },
            {
              \"name\": \"London\",
              \"lat\": 37.129,
              \"lon\": -84.0833,
              \"country\": \"US\",
              \"state\": \"KY\"
            },
            {
              \"name\": \"London\",
              \"lat\": 36.4761,
              \"lon\": -119.4432,
              \"country\": \"US\",
              \"state\": \"CA\"
            }
          ]";
        let text = "
City: London
Country code: GB
Coordinates:
  Latitude: 51.5085
  Longitude: -0.1257

City: London
Country code: CA
Coordinates:
  Latitude: 42.9834
  Longitude: -81.233

City: London
State code: OH
Country code: US
Coordinates:
  Latitude: 39.8865
  Longitude: -83.4483

City: London
State code: KY
Country code: US
Coordinates:
  Latitude: 37.129
  Longitude: -84.0833

City: London
State code: CA
Country code: US
Coordinates:
  Latitude: 36.4761
  Longitude: -119.4432
";
        let res = LocationQueryResponses::new(serde_json::from_str(json).unwrap());
        assert_eq!(res.to_string(), text);
    }
}
