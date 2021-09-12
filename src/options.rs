use crate::units::Units;

#[derive(Debug)]
pub struct Options {
    pub units: Option<Units>,
    pub version: Option<String>,
    pub help: Option<String>,
}
