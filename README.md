# `wethr`

[![CI/CD][ci-cd-badge]][ci-cd-url]
[![Crates.io][crates-badge]][crates-url]
[![MIT license][license-badge]][license-url]

Command line weather tool.

[![Wethr tool][wethr-gif]][wethr-url]

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

- [x] Binaries
- [ ] Tests
- [ ] Proxy support
- [ ] Query by country/city
- [ ] Silent mode

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
[wethr-gif]: https://github.com/risoflora/wethr/raw/master/wethr.gif "Wethr GIF"
[twobucks-wethr-url]: https://github.com/twobucks/wethr "Twobucks's wethr"
