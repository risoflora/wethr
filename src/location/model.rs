#[derive(Clone, Debug)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

impl Coordinates {
    pub fn new(latitude: f32, longitude: f32) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub coordinates: Coordinates,
}
