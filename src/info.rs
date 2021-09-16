use std::fmt::{Display, Formatter, Result};

use crate::{location::model::Location, units::Units, weather::model::Weather};

#[derive(Debug)]
pub struct Info<'a> {
    location: &'a Location,
    weather: &'a Weather,
    units: Units,
    verbose: bool,
}

impl<'a> Info<'a> {
    pub fn new(location: &'a Location, weather: &'a Weather, units: Units) -> Self {
        Self {
            location,
            weather,
            units,
            verbose: false,
        }
    }

    pub fn set_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

impl Display for Info<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let description = format!(
            "{}{}",
            &self.weather.description[0..1].to_uppercase(),
            &self.weather.description[1..]
        );

        let units = self.units.symbol();
        let info = write!(
            f,
            "{city}, {country}: {temperature}{units} {emoji}",
            city = self.location.city,
            country = self.location.country,
            temperature = self.weather.temperature,
            units = units,
            emoji = self.weather.icon
        );
        if self.verbose {
            info.and(write!(
                f,
                "
Weather: {description}
Feels like: {feels_like}{units}
Min: {min}{units}
Max: {max}{units}
Humidity: {humidity}%
Pressure: {pressure} hPa
Sea level: {sea_level} hPa
Ground level: {ground_level} hPa
Clouds: {clouds}%
Wind:
  Speed: {speed}
  Degrees: {degrees}
  Gust: {gust}
Coordinates:
  Longitude: {longitude}
  Latitude: {latitude}
Sunrise: {sunrise}
Sunset: {sunset}
Date/time: {date_time}",
                description = description,
                feels_like = self.weather.feels_like,
                units = units,
                min = self.weather.min_temperature,
                max = self.weather.max_temperature,
                humidity = self.weather.humidity,
                pressure = self.weather.pressure,
                sea_level = self.weather.sea_level,
                ground_level = self.weather.ground_level,
                clouds = self.weather.clouds,
                speed = self.weather.wind.format_speed(self.units),
                degrees = self.weather.wind.degrees,
                gust = self.weather.wind.format_gust(self.units),
                longitude = self.location.coordinates.longitude,
                latitude = self.location.coordinates.latitude,
                sunrise = self.weather.sunrise,
                sunset = self.weather.sunset,
                date_time = self.weather.date_time,
            ))
        } else {
            info
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Info;
    use crate::{
        datetime::DateTime,
        location::model::{Coordinates, Location},
        units::Units::{Celsius, Fahrenheit},
        weather::model::{Weather, Wind},
    };

    #[test]
    fn info_format() {
        let location = Location {
            country: "Brazil".to_string(),
            city: "Monteiro".to_string(),
            coordinates: Coordinates {
                latitude: -7.9194,
                longitude: -37.175,
            },
        };
        let weather = Weather {
            temperature: 25.8,
            icon: "☀️".to_string(),
            description: "scattered clouds".to_string(),
            feels_like: 25.87,
            min_temperature: 25.8,
            max_temperature: 25.8,
            pressure: 1017,
            humidity: 55,
            sea_level: 1017,
            ground_level: 949,
            wind: Wind {
                speed: 4.72,
                degrees: 115,
                gust: 6.14,
            },
            clouds: 46,
            date_time: DateTime::from_unix(1631620646),
            sunrise: DateTime::from_unix(1631607769),
            sunset: DateTime::from_unix(1631651152),
        };
        let units = Celsius;
        let info = Info::new(&location, &weather, units);
        assert_eq!(info.to_string(), "Monteiro, Brazil: 25.8C ☀\u{fe0f}");
        let units = Fahrenheit;
        let info = Info::new(&location, &weather, units);
        assert_eq!(info.to_string(), "Monteiro, Brazil: 25.8F ☀\u{fe0f}");

        let units = Celsius;
        let info = Info::new(&location, &weather, units).set_verbose(true);
        let text = "Monteiro, Brazil: 25.8C ☀\u{fe0f}
Weather: Scattered clouds
Feels like: 25.87C
Min: 25.8C
Max: 25.8C
Humidity: 55%
Pressure: 1017 hPa
Sea level: 1017 hPa
Ground level: 949 hPa
Clouds: 46%
Wind:
  Speed: 4.72 meter/sec
  Degrees: 115
  Gust: 6.14 meter/sec
Coordinates:
  Longitude: -37.175
  Latitude: -7.9194
Sunrise: 2021-09-14T08:22:49Z
Sunset: 2021-09-14T20:25:52Z
Date/time: 2021-09-14T11:57:26Z";
        assert_eq!(info.to_string(), text);
        let units = Fahrenheit;
        let info = Info::new(&location, &weather, units).set_verbose(true);
        let text = "Monteiro, Brazil: 25.8F ☀\u{fe0f}
Weather: Scattered clouds
Feels like: 25.87F
Min: 25.8F
Max: 25.8F
Humidity: 55%
Pressure: 1017 hPa
Sea level: 1017 hPa
Ground level: 949 hPa
Clouds: 46%
Wind:
  Speed: 4.72 miles/hour
  Degrees: 115
  Gust: 6.14 miles/hour
Coordinates:
  Longitude: -37.175
  Latitude: -7.9194
Sunrise: 2021-09-14T08:22:49Z
Sunset: 2021-09-14T20:25:52Z
Date/time: 2021-09-14T11:57:26Z";
        assert_eq!(info.to_string(), text);
    }
}
