use crate::{location::client::LocationProvider, units::Units};

#[derive(Clone, Debug)]
pub struct Options {
    pub units: Option<Units>,
    pub connect_timeout: Option<u64>,
    pub timeout: Option<u64>,
    pub query: Option<String>,
    pub location_provider: Option<LocationProvider>,
    pub full_info: Option<bool>,
    pub silent: Option<bool>,
    pub version: Option<String>,
    pub help: Option<String>,
}
