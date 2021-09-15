#[inline]
fn is_match(text: &str, pattern: &str) -> bool {
    pattern.split("|").any(|d| text.starts_with(d))
}

pub fn get_emoji(description: &str) -> Option<&str> {
    if is_match(description, "clear sky") {
        return Some("☀️");
    }
    if is_match(description, "few clouds") {
        return Some("⛅");
    }
    if is_match(description, "scattered clouds") {
        return Some("☁️");
    }
    if is_match(description, "broken clouds|overcast clouds") {
        return Some("☁️☁️");
    }
    if is_match(description, "thunderstorm") {
        return Some("⛈");
    }
    if is_match(description, "snow|sleet") {
        return Some("🌨");
    }
    if is_match(description, "drizzle|rain") {
        return Some("🌧");
    }
    if is_match(description, "mist|smoke|haze|fog|sand|dust|ash|squalls") {
        return Some("🌫");
    }
    if is_match(description, "tornado|hurricane|tropical storm") {
        return Some("🌪");
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_emoji() {
        assert_eq!(super::get_emoji("clear sky"), Some("☀️"));
        assert_eq!(super::get_emoji("few clouds"), Some("⛅"));
        assert_eq!(super::get_emoji("scattered clouds"), Some("☁️"));
        assert_eq!(super::get_emoji("broken clouds"), Some("☁️☁️"));
        assert_eq!(super::get_emoji("overcast clouds"), Some("☁️☁️"));
        assert_eq!(super::get_emoji("thunderstorm"), Some("⛈"));
        assert_eq!(super::get_emoji("snow"), Some("🌨"));
        assert_eq!(super::get_emoji("sleet"), Some("🌨"));
        assert_eq!(super::get_emoji("drizzle"), Some("🌧"));
        assert_eq!(super::get_emoji("rain"), Some("🌧"));
        assert_eq!(super::get_emoji("mist"), Some("🌫"));
        assert_eq!(super::get_emoji("smoke"), Some("🌫"));
        assert_eq!(super::get_emoji("haze"), Some("🌫"));
        assert_eq!(super::get_emoji("fog"), Some("🌫"));
        assert_eq!(super::get_emoji("sand"), Some("🌫"));
        assert_eq!(super::get_emoji("dust"), Some("🌫"));
        assert_eq!(super::get_emoji("ash"), Some("🌫"));
        assert_eq!(super::get_emoji("squalls"), Some("🌫"));
        assert_eq!(super::get_emoji("tornado"), Some("🌪"));
        assert_eq!(super::get_emoji("hurricane"), Some("🌪"));
        assert_eq!(super::get_emoji("tropical storm"), Some("🌪"));
    }
}
