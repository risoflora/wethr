use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Units {
    Celsius,
    Fahrenheit,
}

impl Default for Units {
    fn default() -> Self {
        Self::Celsius
    }
}

impl Display for Units {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match *self {
            Self::Celsius => write!(fmt, "metric"),
            Self::Fahrenheit => write!(fmt, "imperial"),
        }
    }
}

impl Units {
    pub fn symbol(&self) -> &'static str {
        if *self == Self::Celsius {
            "C"
        } else {
            "F"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Units;

    #[test]
    fn units_to_string() {
        assert_eq!(Units::Celsius.to_string(), "metric");
        assert_eq!(Units::Fahrenheit.to_string(), "imperial");
    }

    #[test]
    fn units_symbol() {
        assert_eq!(Units::Celsius.symbol(), "C");
        assert_eq!(Units::Fahrenheit.symbol(), "F");
    }
}
