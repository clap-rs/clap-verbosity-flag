# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/rust-cli/clap-verbosity-flag/compare/0.2.0...v0.3.0
[0.2.0]: https://github.com/rust-cli/clap-verbosity-flag/compare/v0.1.0...0.2.0
