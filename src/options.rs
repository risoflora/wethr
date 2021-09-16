use crate::units::Units;

#[derive(Debug)]
pub struct Options {
    pub units: Option<Units>,
    pub connect_timeout: Option<u64>,
    pub timeout: Option<u64>,
    pub full_info: Option<bool>,
    pub silent: Option<bool>,
    pub version: Option<String>,
    pub help: Option<String>,
}
