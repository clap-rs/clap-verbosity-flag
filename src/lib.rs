//! Control `log` level with a `--verbose` flag for your CLI
//!
//! # Examples
//!
//! To get `--quiet` and `--verbose` flags through your entire program, just `flatten`
//! [`Verbosity`]:
//! ```rust,no_run
//! # use clap::Parser;
//! # use clap_verbosity_flag::Verbosity;
//! #
//! # /// Le CLI
//! # #[derive(Debug, Parser)]
//! # struct Cli {
//! #[command(flatten)]
//! verbose: Verbosity,
//! # }
//! ```
//!
//! You can then use this to configure your logger:
//! ```rust,no_run
//! # use clap::Parser;
//! # use clap_verbosity_flag::Verbosity;
//! #
//! # /// Le CLI
//! # #[derive(Debug, Parser)]
//! # struct Cli {
//! #     #[command(flatten)]
//! #     verbose: Verbosity,
//! # }
//! let cli = Cli::parse();
//! env_logger::Builder::new()
//!     .filter_level(cli.verbose.log_level_filter())
//!     .init();
//! ```
//!
//! By default, this will only report errors.
//! - `-q` silences output
//! - `-v` show warnings
//! - `-vv` show info
//! - `-vvv` show debug
//! - `-vvvv` show trace
//!
//! By default, the log level is set to Error. To customize this to a different level, pass a type
//! implementing the [`LogLevel`] trait to [`Verbosity`]:
//!
//! ```rust,no_run
//! # use clap::Parser;
//! use clap_verbosity_flag::{Verbosity, InfoLevel};
//!
//! /// Le CLI
//! #[derive(Debug, Parser)]
//! struct Cli {
//!     #[command(flatten)]
//!     verbose: Verbosity<InfoLevel>,
//! }
//! ```
//!
//! Or implement our [`LogLevel`] trait to customize the default log level and help output.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

pub use log::Level;
pub use log::LevelFilter;

/// Logging flags to `#[command(flatten)]` into your CLI
#[derive(clap::Args, Debug, Clone, Default)]
#[command(about = None, long_about = None)]
pub struct Verbosity<L: LogLevel = ErrorLevel> {
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
        global = true,
        help = L::verbose_help(),
        long_help = L::verbose_long_help(),
    )]
    verbose: u8,

    #[arg(
        long,
        short = 'q',
        action = clap::ArgAction::Count,
        global = true,
        help = L::quiet_help(),
        long_help = L::quiet_long_help(),
        conflicts_with = "verbose",
    )]
    quiet: u8,

    #[arg(skip)]
    phantom: std::marker::PhantomData<L>,
}

impl<L: LogLevel> Verbosity<L> {
    /// Create a new verbosity instance by explicitly setting the values
    pub fn new(verbose: u8, quiet: u8) -> Self {
        Verbosity {
            verbose,
            quiet,
            phantom: std::marker::PhantomData,
        }
    }

    /// Whether any verbosity flags (either `--verbose` or `--quiet`)
    /// are present on the command line.
    pub fn is_present(&self) -> bool {
        self.verbose != 0 || self.quiet != 0
    }

    /// Get the log level.
    ///
    /// `None` means all output is disabled.
    pub fn log_level(&self) -> Option<Level> {
        level_enum(self.verbosity())
    }

    /// Get the log level filter.
    pub fn log_level_filter(&self) -> LevelFilter {
        level_enum(self.verbosity())
            .map(|l| l.to_level_filter())
            .unwrap_or(LevelFilter::Off)
    }

    /// If the user requested complete silence (i.e. not just no-logging).
    pub fn is_silent(&self) -> bool {
        self.log_level().is_none()
    }

    fn verbosity(&self) -> u8 {
        let default_verbosity = level_value(L::default());
        let verbosity = default_verbosity as i16 - self.quiet as i16 + self.verbose as i16;
        verbosity.clamp(0, u8::MAX as i16) as u8
    }
}

fn level_value(level: Option<Level>) -> u8 {
    match level {
        None => 0,
        Some(Level::Error) => 1,
        Some(Level::Warn) => 2,
        Some(Level::Info) => 3,
        Some(Level::Debug) => 4,
        Some(Level::Trace) => 5,
    }
}

fn level_enum(verbosity: u8) -> Option<Level> {
    match verbosity {
        0 => None,
        1 => Some(Level::Error),
        2 => Some(Level::Warn),
        3 => Some(Level::Info),
        4 => Some(Level::Debug),
        5..=u8::MAX => Some(Level::Trace),
    }
}

use std::fmt;

impl<L: LogLevel> fmt::Display for Verbosity<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.verbosity())
    }
}

/// Customize the default log-level and associated help
pub trait LogLevel {
    /// Base-line level before applying `--verbose` and `--quiet`
    fn default() -> Option<Level>;

    /// Short-help message for `--verbose`
    fn verbose_help() -> Option<&'static str> {
        Some("Increase logging verbosity")
    }

    /// Long-help message for `--verbose`
    fn verbose_long_help() -> Option<&'static str> {
        None
    }

    /// Short-help message for `--quiet`
    fn quiet_help() -> Option<&'static str> {
        Some("Decrease logging verbosity")
    }

    /// Long-help message for `--quiet`
    fn quiet_long_help() -> Option<&'static str> {
        None
    }
}

/// Default to [`log::Level::Error`]
#[derive(Copy, Clone, Debug, Default)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    fn default() -> Option<Level> {
        Some(Level::Error)
    }
}

/// Default to [`log::Level::Warn`]
#[derive(Copy, Clone, Debug, Default)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    fn default() -> Option<Level> {
        Some(Level::Warn)
    }
}

/// Default to [`log::Level::Info`]
#[derive(Copy, Clone, Debug, Default)]
pub struct InfoLevel;

impl LogLevel for InfoLevel {
    fn default() -> Option<Level> {
        Some(Level::Info)
    }
}

/// Default to [`log::Level::Debug`]
#[derive(Copy, Clone, Debug, Default)]
pub struct DebugLevel;

impl LogLevel for DebugLevel {
    fn default() -> Option<Level> {
        Some(Level::Debug)
    }
}

/// Default to [`log::Level::Trace`]
#[derive(Copy, Clone, Debug, Default)]
pub struct TraceLevel;

impl LogLevel for TraceLevel {
    fn default() -> Option<Level> {
        Some(Level::Trace)
    }
}

/// Default to no logging (i.e. `None` or [`log::LevelFilter::Off`])
#[derive(Copy, Clone, Debug, Default)]
pub struct OffLevel;

impl LogLevel for OffLevel {
    fn default() -> Option<Level> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[command(flatten)]
            verbose: Verbosity,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn verbosity_off_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, None, LevelFilter::Off),
            (1, 0, Some(Level::Error), LevelFilter::Error),
            (2, 0, Some(Level::Warn), LevelFilter::Warn),
            (3, 0, Some(Level::Info), LevelFilter::Info),
            (4, 0, Some(Level::Debug), LevelFilter::Debug),
            (5, 0, Some(Level::Trace), LevelFilter::Trace),
            (6, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, None, LevelFilter::Off),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<OffLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_error_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Error), LevelFilter::Error),
            (1, 0, Some(Level::Warn), LevelFilter::Warn),
            (2, 0, Some(Level::Info), LevelFilter::Info),
            (3, 0, Some(Level::Debug), LevelFilter::Debug),
            (4, 0, Some(Level::Trace), LevelFilter::Trace),
            (5, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, None, LevelFilter::Off),
            (0, 2, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Error), LevelFilter::Error),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<ErrorLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_warn_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Warn), LevelFilter::Warn),
            (1, 0, Some(Level::Info), LevelFilter::Info),
            (2, 0, Some(Level::Debug), LevelFilter::Debug),
            (3, 0, Some(Level::Trace), LevelFilter::Trace),
            (4, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, Some(Level::Error), LevelFilter::Error),
            (0, 2, None, LevelFilter::Off),
            (0, 3, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Warn), LevelFilter::Warn),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<WarnLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_info_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Info), LevelFilter::Info),
            (1, 0, Some(Level::Debug), LevelFilter::Debug),
            (2, 0, Some(Level::Trace), LevelFilter::Trace),
            (3, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, Some(Level::Warn), LevelFilter::Warn),
            (0, 2, Some(Level::Error), LevelFilter::Error),
            (0, 3, None, LevelFilter::Off),
            (0, 4, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Info), LevelFilter::Info),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<InfoLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_debug_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Debug), LevelFilter::Debug),
            (1, 0, Some(Level::Trace), LevelFilter::Trace),
            (2, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, Some(Level::Info), LevelFilter::Info),
            (0, 2, Some(Level::Warn), LevelFilter::Warn),
            (0, 3, Some(Level::Error), LevelFilter::Error),
            (0, 4, None, LevelFilter::Off),
            (0, 5, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Debug), LevelFilter::Debug),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<DebugLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_trace_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::Trace), LevelFilter::Trace),
            (1, 0, Some(Level::Trace), LevelFilter::Trace),
            (255, 0, Some(Level::Trace), LevelFilter::Trace),
            (0, 1, Some(Level::Debug), LevelFilter::Debug),
            (0, 2, Some(Level::Info), LevelFilter::Info),
            (0, 3, Some(Level::Warn), LevelFilter::Warn),
            (0, 4, Some(Level::Error), LevelFilter::Error),
            (0, 5, None, LevelFilter::Off),
            (0, 6, None, LevelFilter::Off),
            (0, 255, None, LevelFilter::Off),
            (255, 255, Some(Level::Trace), LevelFilter::Trace),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<TraceLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }
}
