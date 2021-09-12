# `wethr`

[![Build Status][travis-badge]][travis-url]
[![Crates.io][crates-badge]][crates-url]
[![License][license-badge]][license-url]

Command line weather tool.

![wethr](wethr.gif)

Supported platforms: Linux, Windows and MacOS.

## Installation

```bash
cargo install wethr
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

## Thanks

This project was inspired by [twobucks's wethr][wethr].

## License

This project is licensed under the [MIT license](LICENSE).

[travis-badge]: https://travis-ci.org/risoflora/wethr.svg
[travis-url]: https://travis-ci.org/risoflora/wethr
[crates-badge]: https://img.shields.io/crates/v/wethr.svg
[crates-url]: https://crates.io/crates/wethr
[license-badge]: https://img.shields.io/crates/l/wethr.svg
[license-url]: https://github.com/risoflora/wethr#license
[wethr]: https://github.com/twobucks/wethr
