# clap-verbosity-flag

[![Build Status](https://travis-ci.org/rust-clique/clap-verbosity-flag.svg)][Travis]
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/clap-verbosity-flag.svg)
[![crates.io](https://img.shields.io/crates/v/clap-verbosity-flag.svg)][Crates.io]

[Travis]: https://travis-ci.org/rust-clique/clap-verbosity-flag
[Crates.io]: https://crates.io/crates/clap-verbosity-flag
[Documentation]: https://docs.rs/clap-verbosity-flag/

Easily add a `--verbose` flag to CLIs using Structopt

## Examples

```rust
extern crate clap_verbosity_flag;

// ...
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
```

By default, it'll only report errors.
Passing `-v` one time also prints warnings,
`-vv` enables info logging,
`-vvv` debug,
and `-vvvv` trace.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
