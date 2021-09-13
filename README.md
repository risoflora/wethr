# `wethr`

[![Build status][build-status-badge]][build-status-url]
[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]

Command line weather tool.

![wethr](wethr.gif)

Supported platforms: Linux, Windows and MacOS.

## Installation

```bash
cargo install wethr
```

## Docker

```bash
docker run -it risoflora/wethr
```

## Examples

Get current weather:

```bash
wethr
```

Get current weather in metric units:

```bash
wethr --metric
```

Get current weather in imperial units:

```bash
wethr --imperial
```

## Contributions

Pull Requests and Issues are welcome!

## Whish list

- [ ] Binaries
- [ ] Tests
- [ ] Proxy support
- [ ] Query by country/city
- [ ] Silent mode

## Thanks

This project was inspired by [twobucks's wethr][wethr].

## License

This project is licensed under the [MIT license](LICENSE).

[build-status-badge]: https://github.com/risoflora/wethr/actions/workflows/CI.yml/badge.svg?branch=master
[build-status-url]: https://github.com/risoflora/wethr/actions/workflows/CI.yml
[crates-badge]: https://img.shields.io/crates/v/wethr.svg
[crates-url]: https://crates.io/crates/wethr
[license-badge]: https://img.shields.io/crates/l/wethr.svg
[license-url]: https://github.com/risoflora/wethr#license
[wethr]: https://github.com/twobucks/wethr
