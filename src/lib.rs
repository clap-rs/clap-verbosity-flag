//! Control `log` level with a `--verbose` flag for your CLI
//!
//! # Examples
//!
//! To get `--quiet` and `--verbose` flags through your entire program, just `flatten`
//! [`Verbosity`]:
//!
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
//!
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
//! # #[cfg(feature = "log")]
//! env_logger::Builder::new()
//!     .filter_level(cli.verbose.log_level_filter())
//!     .init();
//!
//! # #[cfg(feature = "tracing")]
//! tracing_subscriber::fmt()
//!     .with_max_level(cli.verbose.tracing_level_filter())
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

use std::fmt;

#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "tracing")]
pub mod tracing;

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
    #[cfg(feature = "log")]
    pub fn log_level(&self) -> Option<log::Level> {
        self.filter().into()
    }

    /// Get the log level filter.
    #[cfg(feature = "log")]
    pub fn log_level_filter(&self) -> log::LevelFilter {
        self.filter().into()
    }

    /// Get the trace level.
    ///
    /// `None` means all output is disabled.
    #[cfg(feature = "tracing")]
    pub fn tracing_level(&self) -> Option<tracing_core::Level> {
        self.filter().into()
    }

    /// Get the trace level filter.
    #[cfg(feature = "tracing")]
    pub fn tracing_level_filter(&self) -> tracing_core::LevelFilter {
        self.filter().into()
    }

    /// If the user requested complete silence (i.e. not just no-logging).
    pub fn is_silent(&self) -> bool {
        self.filter() == Filter::Off
    }

    /// Get the filter level after applying `--verbose` and `--quiet`.
    pub fn filter(&self) -> Filter {
        L::default().with_offset(self.verbose as i16 - self.quiet as i16)
    }
}

/// A representation of the log level filter.
///
/// Used to calculate the log level and filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
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
            _ => Filter::Trace,
        }
    }
}

impl<L: LogLevel> fmt::Display for Verbosity<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.filter() {
            Filter::Off => write!(f, "off"),
            Filter::Error => write!(f, "error"),
            Filter::Warn => write!(f, "warn"),
            Filter::Info => write!(f, "info"),
            Filter::Debug => write!(f, "debug"),
            Filter::Trace => write!(f, "trace"),
        }
    }
}

/// Customize the default log-level and associated help
pub trait LogLevel {
    /// Base-line level before applying `--verbose` and `--quiet`
    fn default() -> Filter;

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

macro_rules! def_log_levels {
    ($($name:ident => $filter:expr,)*) => {
        $(
            #[doc = concat!("An implementation of [`LogLevel`] that defaults to `", stringify!($filter), "`")]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name;

            impl LogLevel for $name {
                fn default() -> Filter {
                    $filter
                }
            }
        )*
    };
}

def_log_levels! {
    OffLevel => Filter::Off,
    ErrorLevel => Filter::Error,
    WarnLevel => Filter::Warn,
    InfoLevel => Filter::Info,
    DebugLevel => Filter::Debug,
    TraceLevel => Filter::Trace,
}

#[cfg(test)]
mod test {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn default_verbosity() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[command(flatten)]
            verbose: Verbosity,
        }

        Cli::command().debug_assert();
    }

    #[test]
    fn verbosity_with_specified_log_level() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[command(flatten)]
            verbose: Verbosity<InfoLevel>,
        }

        Cli::command().debug_assert();
    }
}
