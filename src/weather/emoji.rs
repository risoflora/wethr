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
