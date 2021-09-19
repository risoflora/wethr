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

pub const URL_LOCATION: &str = "https://ipapi.co/json/";

pub const URL_QUERY_LOCATION: &str = "https://api.openweathermap.org/geo/1.0/direct";

#[derive(Debug, Error)]
pub enum LocationClientError {
    #[error(transparent)]
    Client(#[from] ClientError),
    #[error("Wrong query parameter")]
    WrongQueryParam,
    #[error("Returning {0} cities, please choose one:\n{1}")]
    MoreThanOne(usize, String),
}

#[derive(Debug)]
pub struct LocationClient {
    inner: ClientBuilder,
}

#[derive(Clone, Debug, Deserialize)]
struct LocationResponse {
    pub city: Option<String>,
    pub country_name: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}

impl Default for LocationResponse {
    fn default() -> Self {
        Self {
            city: Default::default(),
            country_name: Default::default(),
            latitude: Default::default(),
            longitude: Default::default(),
        }
    }
}

impl From<LocationResponse> for Location {
    fn from(response: LocationResponse) -> Self {
        Self {
            city: response.city.unwrap_or("N/D".to_string()),
            state_code: None,
            country_code: None,
            country: response.country_name,
            coordinates: Coordinates::new(
                response.latitude.unwrap_or_default(),
                response.longitude.unwrap_or_default(),
            ),
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
        if let Some(value) = &self.state_code {
            write!(f, ",{}", value)?;
        }
        if let Some(value) = &self.country_code {
            write!(f, ",{}", value)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct LocationQueryResponse {
    pub name: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lon: Option<f32>,
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

impl From<LocationQueryResponse> for Location {
    fn from(response: LocationQueryResponse) -> Self {
        Self {
            city: response.name.unwrap_or("N/D".to_string()),
            state_code: response.state,
            country_code: response.country,
            country: None,
            coordinates: Coordinates::new(
                response.lat.unwrap_or_default(),
                response.lon.unwrap_or_default(),
            ),
        }
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
            let loc: Location = res.clone().into();
            write!(f, "\n{}\n", loc.to_string())?;
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

    pub async fn get(self) -> Result<Location, LocationClientError> {
        let res: LocationResponse = self.inner.build()?.get(URL_LOCATION).await?;
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
    async fn client_get() {
        sleep(Duration::from_secs(1)).await;
        assert!(LocationClient::new().get().await.is_ok());
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
        assert_eq!(location.country, Some("Brazil".to_string()));
        assert_eq!(location.coordinates.latitude, -7.9194);
        assert_eq!(location.coordinates.longitude, -37.175);
    }

    #[tokio::test]
    async fn client_get_by_query() {
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
        assert_eq!(location.state_code, Some("PB".to_string()));
        assert_eq!(location.country_code, Some("BR".to_string()));
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
