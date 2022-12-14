# BaerScript 🐻

![Build Status](https://github.com/matteopolak/baerscript/actions/workflows/build.yml/badge.svg)
![Release Status](https://github.com/matteopolak/baerscript/actions/workflows/release.yml/badge.svg)
[![License:MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust:Nightly](https://img.shields.io/badge/rust-nightly-blue.svg)](https://www.rust-lang.org/tools/install)

[BaerScript](https://github.com/matteopolak/baerscript) is an esoteric programming language built with Rust 🦀.
Check out the docs [by clicking here](DOCUMENTATION.md).

```bash
Usage: baerscript [OPTIONS] <FILE>
Arguments:
  <FILE>  The path to the baerscript file

Options:
  -d, --debug    Whether to show debugging information
  -a, --ascii    Whether to use ascii
  -h, --help     Print help information
  -V, --version  Print version information
```

## Install

See the [Rust install guide](https://www.rust-lang.org/tools/install) for [Rust nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html),
to [build from source with cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html), and to [run the unit tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

```bash
cargo +nightly build --release
```

### Pre-built Binaries

Binaries are released on major releases for Windows platforms and can be located [in the releases tab](https://github.com/matteopolak/baerscript/releases).

## License

[MIT](./LICENSE)
