# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/rust-cli/clap-verbosity-flag/compare/0.2.0...v0.3.0
[0.2.0]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.1.0...0.2.0
