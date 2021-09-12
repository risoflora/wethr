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
