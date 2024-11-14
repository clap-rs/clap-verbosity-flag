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
//! You can also customize the default logging level:
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
        self.filter().into()
    }

    /// Get the log level filter.
    pub fn log_level_filter(&self) -> LevelFilter {
        self.filter().into()
    }

    /// If the user requested complete silence (i.e. not just no-logging).
    pub fn is_silent(&self) -> bool {
        self.filter() == Filter::Off
    }

    fn filter(&self) -> Filter {
        let offset = self.verbose as i16 - self.quiet as i16;
        Filter::from(L::default()).with_offset(offset)
    }
}

/// An internal representation of the log level filter.
///
/// Used to calculate the log level and filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filter {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Filter {
    /// Apply an offset to the filter level.
    fn with_offset(&self, offset: i16) -> Filter {
        let value = self.as_usize() as i16 + offset;
        const MAX_LEVEL: i16 = 5;
        Self::from_usize(value.clamp(0, MAX_LEVEL) as usize)
    }

    /// Convert the filter to a usize for arithmetic.
    ///
    /// usize avoids negative values (and is used in the log crate).
    fn as_usize(&self) -> usize {
        match self {
            Filter::Off => 0,
            Filter::Error => 1,
            Filter::Warn => 2,
            Filter::Info => 3,
            Filter::Debug => 4,
            Filter::Trace => 5,
        }
    }

    /// Convert a usize back to a filter.
    fn from_usize(value: usize) -> Self {
        match value {
            0 => Filter::Off,
            1 => Filter::Error,
            2 => Filter::Warn,
            3 => Filter::Info,
            4 => Filter::Debug,
            5.. => Filter::Trace,
        }
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Filter::Off => write!(f, "off"),
            Filter::Error => write!(f, "error"),
            Filter::Warn => write!(f, "warn"),
            Filter::Info => write!(f, "info"),
            Filter::Debug => write!(f, "debug"),
            Filter::Trace => write!(f, "trace"),
        }
    }
}

impl From<Filter> for LevelFilter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => LevelFilter::Off,
            Filter::Error => LevelFilter::Error,
            Filter::Warn => LevelFilter::Warn,
            Filter::Info => LevelFilter::Info,
            Filter::Debug => LevelFilter::Debug,
            Filter::Trace => LevelFilter::Trace,
        }
    }
}

impl From<LevelFilter> for Filter {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Off => Filter::Off,
            LevelFilter::Error => Filter::Error,
            LevelFilter::Warn => Filter::Warn,
            LevelFilter::Info => Filter::Info,
            LevelFilter::Debug => Filter::Debug,
            LevelFilter::Trace => Filter::Trace,
        }
    }
}

impl From<Filter> for Option<Level> {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => None,
            Filter::Error => Some(Level::Error),
            Filter::Warn => Some(Level::Warn),
            Filter::Info => Some(Level::Info),
            Filter::Debug => Some(Level::Debug),
            Filter::Trace => Some(Level::Trace),
        }
    }
}

impl From<Option<Level>> for Filter {
    fn from(level: Option<Level>) -> Self {
        match level {
            None => Filter::Off,
            Some(Level::Error) => Filter::Error,
            Some(Level::Warn) => Filter::Warn,
            Some(Level::Info) => Filter::Info,
            Some(Level::Debug) => Filter::Debug,
            Some(Level::Trace) => Filter::Trace,
        }
    }
}

use std::fmt;

impl<L: LogLevel> fmt::Display for Verbosity<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.filter())
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
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    fn default() -> Option<Level> {
        Some(Level::Error)
    }
}

/// Default to [`log::Level::Warn`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    fn default() -> Option<Level> {
        Some(Level::Warn)
    }
}

/// Default to [`log::Level::Info`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct InfoLevel;

impl LogLevel for InfoLevel {
    fn default() -> Option<Level> {
        Some(Level::Info)
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
}
