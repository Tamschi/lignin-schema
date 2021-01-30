# lignin-schema

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/lignin-schema)
[![Crates.io](https://img.shields.io/crates/v/lignin-schema)](https://crates.io/crates/lignin-schema)
[![Docs.rs](https://docs.rs/lignin-schema/badge.svg)](https://docs.rs/crates/lignin-schema)

![Rust 1.45.0](https://img.shields.io/static/v1?logo=Rust&label=&message=1.45.0&color=grey)
[![CI](https://github.com/Tamschi/lignin-schema/workflows/CI/badge.svg?branch=unstable)](https://github.com/Tamschi/lignin-schema/actions?query=workflow%3ACI+branch%3Aunstable)
![Crates.io - License](https://img.shields.io/crates/l/lignin-schema/0.0.4)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/lignin-schema)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/lignin-schema)](https://github.com/Tamschi/lignin-schema/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/lignin-schema)](https://github.com/Tamschi/lignin-schema/pulls)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/lignin-schema.svg)](https://web.crev.dev/rust-reviews/crate/lignin-schema/)

An HTML schema interface through which [lignin] Node trees can be created with efficient compile-time checking.

[lignin]: https://github.com/Tamschi/lignin

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add lignin-schema
```

## Example

```rust
// TODO_EXAMPLE
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`lignin-schema` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

* The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
* The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
