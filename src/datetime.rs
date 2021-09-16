use std::{
    fmt::{Display, Formatter, Result},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone)]
pub struct DateTime(SystemTime);

impl DateTime {
    pub fn from_unix(value: u64) -> Self {
        Self {
            0: UNIX_EPOCH + Duration::from_secs(value),
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            humantime::format_rfc3339_seconds(self.0).to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::DateTime;

    #[test]
    fn format_unix_datetime() {
        let datetime = DateTime::from_unix(1631620646);
        assert_eq!(datetime.to_string(), "2021-09-14T11:57:26Z");
    }
}
