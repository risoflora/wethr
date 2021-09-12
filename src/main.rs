use std::{process, time::Duration};

use wethr::{
    args,
    client::{CLIENT_CONNECT_TIMEOUT, CLIENT_TIMEOUT},
    location::client::LocationClient,
    spinner::Spinner,
    weather::client::WeatherClient,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = args::Args::parse_from_env()?;
    if let Some(text) = opts.help.or(opts.version) {
        println!("{}", text);
        process::exit(0);
    }
    let spinner = Spinner::new();
    let connect_timeout = opts.connect_timeout.unwrap_or(CLIENT_CONNECT_TIMEOUT);
    let timeout = opts.timeout.unwrap_or(CLIENT_TIMEOUT);
    let location = spinner
        .set_message("Detecting your location")
        .run(
            LocationClient::new()
                .set_connect_timeout(Duration::from_secs(connect_timeout))
                .set_timeout(Duration::from_secs(timeout))
                .get(),
        )
        .await?;
    let units = opts.units.unwrap_or_default();
    let weather = spinner
        .set_color("yellow")
        .set_message("Loading weather")
        .run(
            WeatherClient::new()
                .set_connect_timeout(Duration::from_secs(connect_timeout))
                .set_timeout(Duration::from_secs(timeout))
                .get_with_units(&location.coordinates, units),
        )
        .await?;
    spinner.println(format!(
        "{city}, {country}: {temperature}{units} {emoji}",
        city = location.city,
        country = location.country,
        temperature = weather.temperature,
        units = units.symbol(),
        emoji = weather.icon
    ));
    Ok(())
}
