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
    pub state_code: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub coordinates: Coordinates,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "City: {}\n", &self.city)?;
        if let Some(state_code) = &self.state_code {
            write!(f, "State code: {}\n", state_code)?;
        }
        if let Some(country_code) = &self.country_code {
            write!(f, "Country code: {}\n", country_code)?;
        }
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
            city: "Monteiro".to_string(),
            state_code: Some("PB".to_string()),
            country_code: Some("BR".to_string()),
            country: None,
            coordinates: Coordinates::new(12.34, 56.78),
        };
        let text = "City: Monteiro
State code: PB
Country code: BR
Coordinates:
  Latitude: 12.34
  Longitude: 56.78";
        assert_eq!(location.to_string(), text);

        let location = Location {
            city: "Monteiro".to_string(),
            state_code: None,
            country_code: Some("BR".to_string()),
            country: None,
            coordinates: Coordinates::new(12.34, 56.78),
        };
        let text = "City: Monteiro
Country code: BR
Coordinates:
  Latitude: 12.34
  Longitude: 56.78";
        assert_eq!(location.to_string(), text);

        let location = Location {
            city: "Monteiro".to_string(),
            state_code: None,
            country_code: None,
            country: None,
            coordinates: Coordinates::new(12.34, 56.78),
        };
        let text = "City: Monteiro
Coordinates:
  Latitude: 12.34
  Longitude: 56.78";
        assert_eq!(location.to_string(), text);
    }
}
