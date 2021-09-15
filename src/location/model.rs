use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LocationResponse {
    pub country: String,
    pub city: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug)]
pub struct Location {
    pub country: String,
    pub city: String,
    pub coordinates: Coordinates,
}

impl From<LocationResponse> for Location {
    fn from(response: LocationResponse) -> Self {
        Self {
            country: response.country,
            city: response.city,
            coordinates: Coordinates {
                latitude: response.lat,
                longitude: response.lon,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, LocationResponse};

    #[test]
    fn location_from_response() {
        let json = "{
            \"country\": \"Brazil\",
            \"city\": \"Monteiro\",
            \"lat\": -7.9194,
            \"lon\": -37.175
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
