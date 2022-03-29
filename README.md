# `wethr`

[![CI/CD][ci-cd-badge]][ci-cd-url]
[![Crates.io][crates-badge]][crates-url]
[![MIT license][license-badge]][license-url]

Command line weather tool.

[![Wethr tool][wethr-gif]][wethr-url]

## Usage

```
Usage: wethr [options] [city name[,state code][,country code]]

Options:
    -m, --metric        Weather in metric units (compatibility)
    -i, --imperial      Weather in imperial units (compatibility)
    -u, --unit [C]elsius or [F]ahrenheit
                        Unit of measurement
    -c, --connect-timeout 5
                        Connect timeout (in seconds)
    -t, --timeout 30    Timeout (in seconds)
    -p, --location-provider 0 to 3
                        Location provider
    -f, --full-info     Full weather information
    -s, --silent        Silent mode
    -v, --version       Print program version
    -h, --help          Print this help menu
```

## Download

Stable binaries for Linux, Windows and Mac OS are available for download at the
[releases page](https://github.com/risoflora/wethr/releases).

## Docker

```bash
docker run -it risoflora/wethr
```

## Installation

```bash
cargo install wethr
```

## Contributions

Pull Requests and Issues are welcome!

## Wish list

- [x] Binaries
- [x] Tests
- [x] Silent mode
- [x] Full info
- [x] Query by city name\[,state code\[,country code\]\]
- [x] Location provider
- [ ] Proxy support
- [ ] Configure the [OWM](https://openweathermap.org) API token
- [ ] Template support (e.g.
      `wethr -t '{city} - Temperature: {temperature} - Humidity: {humidity}'`)
- [ ] Forecast support

## Thanks

This project was inspired by [twobucks's wethr][twobucks-wethr-url].

## License

This project is licensed under the [MIT license](LICENSE).

[ci-cd-badge]: https://img.shields.io/github/workflow/status/risoflora/wethr/CI?style=flat-square "CI/CD"
[ci-cd-url]: https://github.com/risoflora/wethr/actions/workflows/CI.yml "GitHub actions"
[crates-badge]: https://img.shields.io/crates/v/wethr.svg?style=flat-square
[crates-url]: https://crates.io/crates/wethr "Wethr crate"
[license-badge]: https://img.shields.io/crates/l/wethr.svg?style=flat-square
[license-url]: https://github.com/risoflora/wethr/blob/master/LICENSE "MIT license"
[wethr-url]: https://github.com/risoflora/wethr "Wethr tool"
[wethr-gif]: https://github.com/risoflora/wethr/raw/main/wethr.gif "Wethr GIF"
[twobucks-wethr-url]: https://github.com/twobucks/wethr "Twobucks's wethr"
