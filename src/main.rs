use std::{process, time::Duration};

use wethr::{
    args,
    client::{CLIENT_CONNECT_TIMEOUT, CLIENT_TIMEOUT},
    info::Info,
    location::client::LocationClient,
    spinner::{Spinner, SpinnerColor},
    weather::client::WeatherClient,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = args::Args::parse_from_env()?;
    if let Some(text) = opts.help.or(opts.version) {
        println!("{}", text);
        process::exit(0);
    }
    let spinner = Spinner::new().set_silent(opts.silent.is_some());
    let connect_timeout = opts.connect_timeout.unwrap_or(CLIENT_CONNECT_TIMEOUT);
    let timeout = opts.timeout.unwrap_or(CLIENT_TIMEOUT);
    let location = spinner
        .set_color(SpinnerColor::Blue)
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
        .set_color(SpinnerColor::Yellow)
        .set_message("Loading weather")
        .run(
            WeatherClient::new()
                .set_connect_timeout(Duration::from_secs(connect_timeout))
                .set_timeout(Duration::from_secs(timeout))
                .get_with_units(&location.coordinates, units),
        )
        .await?;
    let info = Info::new(&location, &weather, units).set_verbose(opts.full_info.is_some());
    spinner.print_message(&info.to_string());
    Ok(())
}
