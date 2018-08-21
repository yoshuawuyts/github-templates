# github-templates
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Generate GitHub issue templates.

- [Documentation][8]
- [Crates.io][2]
- [Releases][9]

## Screenshot
![screenshot](./assets/screenshot.png)

## Usage
```txt
github-templates 0.1.0
Yoshua Wuyts <yoshuawuyts@gmail.com>
Generate GitHub issue templates.

USAGE:
    github-templates [FLAGS] [OPTIONS] [dir]

FLAGS:
    -h, --help         Prints help information
    -P, --pretty       Enable pretty printing
    -V, --version      Prints version information
    -v, --verbosity    Pass many times for more log output

OPTIONS:
    -n, --name <name>    Project name. Defaults to target directory name

ARGS:
    <dir>    Target directory [default: .]
```

## Acknowledgements
The templates included are based on [Parcel's excellent issue
templates](https://github.com/parcel-bundler/parcel/tree/master/.github/ISSUE_TEMPLATE),
and [Rust's RFC
template](https://github.com/rust-lang/rfcs/blob/master/0000-template.md).

## Installation
```sh
$ cargo add github-templates
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/github-templates.svg?style=flat-square
[2]: https://crates.io/crates/github-templates
[3]: https://img.shields.io/travis/yoshuawuyts/github-templates.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/github-templates
[5]: https://img.shields.io/crates/d/github-templates.svg?style=flat-square
[6]: https://crates.io/crates/github-templates
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/github-templates
[9]: https://github.com/yoshuawuyts/github-templates/releases
