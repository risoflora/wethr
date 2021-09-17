#[derive(Clone, Debug)]
pub struct Coordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Clone, Debug)]
pub struct Location {
    pub country: String,
    pub city: String,
    pub coordinates: Coordinates,
}
