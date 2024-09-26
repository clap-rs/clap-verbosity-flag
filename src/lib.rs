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

    fn verbosity(&self) -> i8 {
        level_value(L::default()) - (self.quiet as i8) + (self.verbose as i8)
    }
}

fn level_value(level: Option<Level>) -> i8 {
    match level {
        None => 0,
        Some(Level::Error) => 1,
        Some(Level::Warn) => 2,
        Some(Level::Info) => 3,
        Some(Level::Debug) => 4,
        Some(Level::Trace) => 5,
    }
}

fn level_enum(verbosity: i8) -> Option<Level> {
    match verbosity {
        i8::MIN..=0 => None,
        1 => Some(Level::Error),
        2 => Some(Level::Warn),
        3 => Some(Level::Info),
        4 => Some(Level::Debug),
        5..=i8::MAX => Some(Level::Trace),
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
            (255, 0, None, LevelFilter::Off),
            (0, 1, None, LevelFilter::Off),
            (0, 2, None, LevelFilter::Off),
            (0, 255, Some(Level::Warn), LevelFilter::Warn),
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
            (255, 0, Some(Level::Error), LevelFilter::Error),
            (0, 1, Some(Level::Error), LevelFilter::Error),
            (0, 2, None, LevelFilter::Off),
            (0, 3, None, LevelFilter::Off),
            (0, 255, Some(Level::Info), LevelFilter::Info),
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
            (255, 0, Some(Level::Warn), LevelFilter::Warn),
            (0, 1, Some(Level::Warn), LevelFilter::Warn),
            (0, 2, Some(Level::Error), LevelFilter::Error),
            (0, 3, None, LevelFilter::Off),
            (0, 4, None, LevelFilter::Off),
            (0, 255, Some(Level::Debug), LevelFilter::Debug),
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
