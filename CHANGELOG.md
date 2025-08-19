# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [3.0.4] - 2025-08-19

### Features

- Add `serde` feature for when mixing CLI and config
- Add comparison operators to types
- Add conversions between `Verbosity` and `VerbosityFilter`

## [3.0.3] - 2025-05-20

## [3.0.2] - 2024-12-16

### Features

- Add a `Copy` impl to `Verbosity`

## [3.0.1] - 2024-11-26

### Features

- Add `impl From<Verbosity>` for logging level and level-filter types

## [3.0.0] - 2024-11-20

### Breaking Changes

- Log support put behind the default `log` feature
- `LogLevel::default` renamed to `LogLevel::default_filter` and now returns `VerbosityFilter`
- `Level` and `LevelFilter` are moved into the `log` mod

### Features

- Add `tracing` feature with `Verbosity::tracing_level` and `Verbosity::tracing_level_filter`

## [2.2.3] - 2024-11-16

### Features

- Add `DebugLevel` and `TraceLevel` for exploratory programming

## [2.2.2] - 2024-09-26

### Fixes

- Resolve overflow issues

## [2.2.1] - 2024-07-25

### Compatibility

- Raised MSRV to 1.74

## [2.2.0] - 2024-02-14

### Compatibility

- Raised MSRV to 1.73

### Features

- Add `is_present()` to see if one of the flags is present on the command-line

## [2.1.2] - 2024-01-16

### Fixes

- Don't pollute the CLIs help

## [2.1.1] - 2023-12-15

### Fixes

- Tried to clarify help output for `-v` / `-q`

### Documentation

- Tried to clarify role of `LogLevel` trait

## [2.1.0] - 2023-10-23

### Compatibility

- Raised MSRV to 1.70.0

### Features

- Implement `Default`

## [2.0.1] - 2023-03-29

### Features

- Re-export enums from `log`

## [2.0.0] - 2022-09-28

### Breaking Changes

- Upgraded to clap v4

### Compatibility

- Raised MSRV to 1.60.0

## [1.0.1] - 2022-06-13

### Fixes

- Clarify relationship with `log`

## [1.0.0] - 2022-02-09

### Breaking Changes

- `set_default`has been removed in favor of being generic over `LogLevel`.
- `--verbose` and `--quiet` are now global

### Features

- Allow customizing help text

### Fixes

- `--verbose` and `--quiet` are now global

## [0.4.1] - 2022-01-27

### Features

- Added `log_level_filter` for convenience

### Fixes

- `Display` now shows the combination of quiet and verbose
- Improved examples in documentation

## [0.4.0] - 2021-12-31

### Breaking Changes

- Upgraded to clap3

## [0.3.2] - 2021-08-07

### Added

- Allow instantiating the struct

## [0.3.1] - 2020-01-16

### Fixed

- Workaround bug in structopt causing `clap-verbosity-flag`s doc-comment to win out over the caller's

## [0.3.0] - 2019-10-17
### Added
- Support `.set_default_level()` for configuring the default level
- `--quiet` is now supported
- Support Logging being disabled via `None`

### Breaking Changes
- `structopt` 0.3 is now required
- Removed `setuop_env_logger`. requiring you to configure the logger yourself.
- `--verbosiy` changed to `--verbose`
- `log_level` now returns an `Option` with the intention that `None` means no user-visible outpiut (including logging)

## [0.2.0] - 2017-06-02

<!-- next-url -->
[Unreleased]: https://github.com/clap-rs/clap-verbosity-flag/compare/v3.0.4...HEAD
[3.0.4]: https://github.com/clap-rs/clap-verbosity-flag/compare/v3.0.3...v3.0.4
[3.0.3]: https://github.com/clap-rs/clap-verbosity-flag/compare/v3.0.2...v3.0.3
[3.0.2]: https://github.com/clap-rs/clap-verbosity-flag/compare/v3.0.1...v3.0.2
[3.0.1]: https://github.com/clap-rs/clap-verbosity-flag/compare/v3.0.0...v3.0.1
[3.0.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.2.3...v3.0.0
[2.2.3]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.2.2...v2.2.3
[2.2.2]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.2.1...v2.2.2
[2.2.1]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.2.0...v2.2.1
[2.2.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.1.2...v2.2.0
[2.1.2]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.1.1...v2.1.2
[2.1.1]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.1.0...v2.1.1
[2.1.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.0.1...v2.1.0
[2.0.1]: https://github.com/clap-rs/clap-verbosity-flag/compare/v2.0.0...v2.0.1
[2.0.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/v1.0.1...v2.0.0
[1.0.1]: https://github.com/clap-rs/clap-verbosity-flag/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/v0.4.1...v1.0.0
[0.4.1]: https://github.com/clap-rs/clap-verbosity-flag/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/clap-rs/clap-verbosity-flag/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/clap-rs/clap-verbosity-flag/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/0.2.0...v0.3.0
[0.2.0]: https://github.com/clap-rs/clap-verbosity-flag/compare/v0.1.0...0.2.0
