# lignin-schema

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/lignin-schema)
[![Crates.io](https://img.shields.io/crates/v/lignin-schema)](https://crates.io/crates/lignin-schema)
[![Docs.rs](https://docs.rs/lignin-schema/badge.svg)](https://docs.rs/lignin-schema)

![Rust 1.45](https://img.shields.io/static/v1?logo=Rust&label=&message=1.45&color=grey)
[![CI](https://github.com/Tamschi/lignin-schema/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/lignin-schema/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/lignin-schema/0.0.4)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/lignin-schema)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/lignin-schema)](https://github.com/Tamschi/lignin-schema/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/lignin-schema)](https://github.com/Tamschi/lignin-schema/pulls)
[![good first issues](https://img.shields.io/github/issues-raw/Tamschi/lignin-schema/good%20first%20issue?label=good+first+issues)](https://github.com/Tamschi/lignin-schema/contribute)

[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/lignin-schema.svg)](https://web.crev.dev/rust-reviews/crate/lignin-schema/)

Partial HTML, MathML and SVG schemas as static APIs, with documentation and deprecation attributes.

This can be used by macro crates to implement errors, warnings and hover docs, without the schema information being available to and/or processed by the macro itself.

Works well alongside [lignin](https://github.com/Tamschi/lignin) but does not depend on it.

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

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`lignin-schema` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

- The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
- The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).
