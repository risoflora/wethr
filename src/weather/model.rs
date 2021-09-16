use crate::{datetime::DateTime, units::Units};

#[derive(Debug)]
pub struct Wind {
    pub speed: f32,
    pub degrees: i32,
    pub gust: f32,
}

impl Wind {
    pub fn format_metric(units: Units) -> &'static str {
        if units == Units::Celsius {
            "meter/sec"
        } else {
            "miles/hour"
        }
    }

    pub fn format_speed(&self, units: Units) -> String {
        format!("{} {}", self.speed, Wind::format_metric(units))
    }

    pub fn format_gust(&self, units: Units) -> String {
        format!("{} {}", self.gust, Wind::format_metric(units))
    }
}

#[derive(Debug)]
pub struct Weather {
    pub temperature: f32,
    pub icon: String,
    pub description: String,
    pub feels_like: f32,
    pub min_temperature: f32,
    pub max_temperature: f32,
    pub pressure: i32,
    pub humidity: i32,
    pub sea_level: i32,
    pub ground_level: i32,
    pub wind: Wind,
    pub clouds: i32,
    pub date_time: DateTime,
    pub sunrise: DateTime,
    pub sunset: DateTime,
}

#[cfg(test)]
mod tests {
    use super::Wind;
    use crate::units::Units::*;

    #[test]
    fn wind_format() {
        assert_eq!(Wind::format_metric(Celsius), "meter/sec");
        assert_eq!(Wind::format_metric(Fahrenheit), "miles/hour");

        let wind = Wind {
            speed: 4.72,
            degrees: 115,
            gust: 6.14,
        };
        assert_eq!(wind.format_speed(Celsius), "4.72 meter/sec");
        assert_eq!(wind.format_speed(Fahrenheit), "4.72 miles/hour");
        assert_eq!(wind.format_gust(Celsius), "6.14 meter/sec");
        assert_eq!(wind.format_gust(Fahrenheit), "6.14 miles/hour");
    }
}
