use std::{env, result::Result};

use getopts::{Fail, Matches, Options as OptsOptions};
use thiserror::Error;

use crate::{consts, options::Options, units::Units};

#[derive(Error, Debug)]
pub enum ArgsError {
    #[error(transparent)]
    GetOpts(#[from] Fail),
}

pub struct Args(Options);

pub type ArgsResult = Result<Options, ArgsError>;

impl Args {
    #[inline]
    fn new() -> OptsOptions {
        let mut opts = OptsOptions::new();
        opts.optflag("m", "metric", "Weather in metric units (compatibility)")
            .optflag("i", "imperial", "Weather in imperial units (compatibility)")
            .optopt(
                "u",
                "unit",
                "Unit of measurement",
                "[C]elsius or [F]ahrenheit",
            )
            .optopt("c", "connect-timeout", "Connect timeout (in seconds)", "5")
            .optopt("t", "timeout", "Timeout (in seconds)", "30")
            .optflag("v", "version", "Print program version")
            .optflag("h", "help", "Print this help menu");
        opts
    }

    #[inline]
    fn parse_units(matches: &Matches) -> Option<Units> {
        if matches.opt_present("m") {
            Some(Units::Celsius)
        } else if matches.opt_present("i") {
            Some(Units::Fahrenheit)
        } else {
            let units = matches.opt_str("u").unwrap_or_default();
            if units.eq_ignore_ascii_case("C") {
                Some(Units::Celsius)
            } else if units.eq_ignore_ascii_case("F") {
                Some(Units::Fahrenheit)
            } else {
                None
            }
        }
    }

    #[inline]
    fn parse_connect_timeout(matches: &Matches) -> Option<u64> {
        matches.opt_get("c").unwrap_or_default()
    }

    #[inline]
    fn parse_timeout(matches: &Matches) -> Option<u64> {
        matches.opt_get("t").unwrap_or_default()
    }

    #[inline]
    fn parse_version(matches: &Matches) -> Option<String> {
        if matches.opt_present("v") {
            Some(consts::PROGRAM_VERSION.to_owned())
        } else {
            None
        }
    }

    #[inline]
    fn parse_help(opts: &OptsOptions, matches: &Matches) -> Option<String> {
        if matches.opt_present("h") {
            Some(opts.usage(&format!("Usage: {} [options]", consts::PROGRAM_NAME)))
        } else {
            None
        }
    }

    pub fn parse(args: &[String]) -> ArgsResult {
        let opts = Self::new();
        let matches = opts.parse(args)?;
        let args = Self {
            0: Options {
                units: Self::parse_units(&matches),
                connect_timeout: Self::parse_connect_timeout(&matches),
                timeout: Self::parse_timeout(&matches),
                version: Self::parse_version(&matches),
                help: Self::parse_help(&opts, &matches),
            },
        };
        Ok(args.0)
    }

    pub fn parse_from_env() -> ArgsResult {
        let args: Vec<String> = env::args().collect();
        Self::parse(&args[1..])
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use crate::units::Units;

    #[test]
    fn args_parse_unit() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.units, None);

        let opt = Args::parse(&["--metric".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));
        let opt = Args::parse(&["-m".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));

        let opt = Args::parse(&["--imperial".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
        let opt = Args::parse(&["-i".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));

        let opt = Args::parse(&["--unit=C".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));
        let opt = Args::parse(&["-uC".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));
        let opt = Args::parse(&["-uc".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Celsius));

        let opt = Args::parse(&["--unit=F".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
        let opt = Args::parse(&["-uF".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
        let opt = Args::parse(&["-uf".to_string()]).unwrap();
        assert_eq!(opt.units, Some(Units::Fahrenheit));
    }

    #[test]
    fn args_parse_timeouts() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.connect_timeout, None);

        let opt = Args::parse(&["--connect-timeout=123".to_string()]).unwrap();
        assert_eq!(opt.connect_timeout, Some(123));
        let opt = Args::parse(&["-c123".to_string()]).unwrap();
        assert_eq!(opt.connect_timeout, Some(123));

        assert_eq!(opt.timeout, None);
        let opt = Args::parse(&["--timeout=123".to_string()]).unwrap();
        assert_eq!(opt.timeout, Some(123));
        let opt = Args::parse(&["-t123".to_string()]).unwrap();
        assert_eq!(opt.timeout, Some(123));
    }

    #[test]
    fn args_parse_version() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.version, None);

        let opt = Args::parse(&["--version".to_string()]).unwrap();
        assert_eq!(opt.version, Some(env!("CARGO_PKG_VERSION").to_string()));
        let opt = Args::parse(&["-v".to_string()]).unwrap();
        assert_eq!(opt.version, Some(env!("CARGO_PKG_VERSION").to_string()));
    }

    #[test]
    fn args_parse_help() {
        let opt = Args::parse(&[]).unwrap();
        assert_eq!(opt.help, None);

        let help = "Usage: wethr [options]

Options:
    -m, --metric        Weather in metric units (compatibility)
    -i, --imperial      Weather in imperial units (compatibility)
    -u, --unit [C]elsius or [F]ahrenheit
                        Unit of measurement
    -c, --connect-timeout 5
                        Connect timeout (in seconds)
    -t, --timeout 30    Timeout (in seconds)
    -v, --version       Print program version
    -h, --help          Print this help menu
";
        let opt = Args::parse(&["--help".to_string()]).unwrap();
        assert_eq!(opt.help, Some(help.to_string()));
        let opt = Args::parse(&["-h".to_string()]).unwrap();
        assert_eq!(opt.help, Some(help.to_string()));
    }
}
