# clap-verbosity-flag for `log` / `tracing`

[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/clap-verbosity-flag.svg)
[![crates.io](https://img.shields.io/crates/v/clap-verbosity-flag.svg)][Crates.io]

[Crates.io]: https://crates.io/crates/clap-verbosity-flag
[Documentation]: https://docs.rs/clap-verbosity-flag/

Easily add `--verbose` and `--quiet` flags to CLIs using [Clap](http://crates.io/crates/clap).

## Examples

```console
$ cargo add clap-verbosity-flag
```

```rust
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbosity.into())
        .init();
    // Your code here
}
```

For [`tracing`](https://crates.io/crates/tracing) support, use the `tracing` feature:

```console
$ cargo add clap-verbosity-flag --no-default-features --features tracing
```

```rust
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .init();
    // Your code here
}
```

The default verbosity level will cause `log` / `tracing` to only report errors. The flags can be
specified multiple times to increase or decrease the verbosity level. See the [Documentation] for
info on how to change the default verbosity level.

- silence output: `-q` / `--quiet`
- show warnings: `-v` / `--verbose`
- show info: `-vv` / `--verbose --verbose`
- show debug: `-vvv` / `--verbose --verbose --verbose`
- show trace: `-vvvv` / `--verbose --verbose --verbose --verbose`

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual-licensed as above, without any additional terms or
conditions.
