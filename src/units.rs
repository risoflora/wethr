use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
