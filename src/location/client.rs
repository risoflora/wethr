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
}

#[derive(Debug)]
pub struct LocationClient {
    inner: ClientBuilder,
}

#[derive(Clone, Debug, Deserialize)]
struct LocationResponse {
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}

impl Default for LocationResponse {
    fn default() -> Self {
        Self {
            country: Default::default(),
            city: Default::default(),
            latitude: Default::default(),
            longitude: Default::default(),
        }
    }
}

impl From<LocationResponse> for Location {
    fn from(response: LocationResponse) -> Self {
        Self {
            country: response.country.unwrap_or("N/D".to_uppercase()),
            city: response.city.unwrap_or("N/D".to_uppercase()),
            coordinates: Coordinates {
                latitude: response.latitude.unwrap_or_default(),
                longitude: response.longitude.unwrap_or_default(),
            },
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
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lon: Option<f32>,
}

impl Default for LocationQueryResponse {
    fn default() -> Self {
        Self {
            name: Default::default(),
            country: Default::default(),
            lat: Default::default(),
            lon: Default::default(),
        }
    }
}

impl From<LocationQueryResponse> for Location {
    fn from(response: LocationQueryResponse) -> Self {
        Self {
            country: response.country.unwrap_or("N/D".to_uppercase()),
            city: response.name.unwrap_or("N/D".to_uppercase()),
            coordinates: Coordinates {
                latitude: response.lat.unwrap_or_default(),
                longitude: response.lon.unwrap_or_default(),
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

    pub async fn get_by_query(
        self,
        query: &LocationQuery,
    ) -> Result<Location, LocationClientError> {
        if query.city_name.len() < 2 {
            return Err(LocationClientError::WrongQueryParam);
        };
        let url = format!(
            "{url}?q={query}&limit=1&appid={appid}",
            url = URL_QUERY_LOCATION,
            query = query,
            appid = TOKEN
        );
        let res: Vec<LocationQueryResponse> = self.inner.build()?.get(&url).await?;
        let loc = res
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

    use super::{Location, LocationClient, LocationQuery, LocationQueryResponse, LocationResponse};

    #[tokio::test]
    async fn client_get() {
        sleep(Duration::from_secs(1)).await;
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

    #[tokio::test]
    async fn client_get_by_query() {
        sleep(Duration::from_secs(1)).await;
        let query = LocationQuery::from("monteiro".to_string());
        assert!(LocationClient::new().get_by_query(&query).await.is_ok());

        let query = LocationQuery::from("".to_string());
        assert!(LocationClient::new().get_by_query(&query).await.is_err());
    }

    #[test]
    fn location_from_query_response() {
        let json = "{
                \"name\": \"Monteiro\",
                \"country\": \"Brazil\",
                \"lat\": -7.9194,
                \"lon\": -37.175
            }";
        let response = serde_json::from_str::<LocationQueryResponse>(json);
        assert!(response.is_ok());
        let location: Location = response.unwrap().into();
        assert_eq!(location.city, "Monteiro");
        assert_eq!(location.country, "Brazil");
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
}
