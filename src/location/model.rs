use std::fmt::{Display, Formatter, Result};

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

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "City: {}\n", &self.city)?;
        write!(f, "Country code: {}\n", &self.country)?;
        write!(f, "Coordinates:\n")?;
        write!(f, "  Latitude: {}\n", &self.coordinates.latitude)?;
        write!(f, "  Longitude: {}", &self.coordinates.longitude)
    }
}

#[cfg(test)]
mod tests {
    use super::{Coordinates, Location};

    #[test]
    fn location_display() {
        let location = Location {
            country: "BR".to_string(),
            city: "Monteiro".to_string(),
            coordinates: Coordinates::new(12.34, 56.78),
        };
        let text = "City: Monteiro
Country code: BR
Coordinates:
  Latitude: 12.34
  Longitude: 56.78";
        assert_eq!(location.to_string(), text);
    }
}
