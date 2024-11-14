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

/// These types are re-exported for backwards compatibility only.
#[cfg(any(doc, feature = "log"))]
#[doc(hidden)]
pub use self::log::{ErrorLevel, InfoLevel, WarnLevel};

#[cfg(any(doc, feature = "log"))]
pub mod log;

#[cfg(any(doc, feature = "tracing"))]
pub mod tracing;

/// Logging flags to `#[command(flatten)]` into your CLI
#[cfg(any(doc, feature = "log"))]
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

/// Logging flags to `#[command(flatten)]` into your CLI
#[cfg(not(any(doc, feature = "log")))]
#[derive(clap::Args, Debug, Clone, Default)]
#[command(about = None, long_about = None)]
pub struct Verbosity<L: LogLevel> {
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

impl<L: LogLevel> Verbosity<L>
where
    Filter: Into<Option<L::Level>> + Into<L::LevelFilter> + From<Option<L::Level>>,
{
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
    pub fn log_level(&self) -> Option<L::Level> {
        self.filter().into()
    }

    /// Get the log level filter.
    pub fn log_level_filter(&self) -> L::LevelFilter {
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

use std::fmt;

impl<L: LogLevel> fmt::Display for Verbosity<L>
where
    Filter: Into<Option<L::Level>> + Into<L::LevelFilter> + From<Option<L::Level>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.filter())
    }
}

/// Customize the default log-level and associated help
pub trait LogLevel {
    type Level;
    type LevelFilter;

    /// Base-line level before applying `--verbose` and `--quiet`
    fn default() -> Option<Self::Level>;

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

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(feature = "log")]
    fn default_verbosity() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[command(flatten)]
            verbose: Verbosity,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    #[cfg(feature = "log")]
    fn verbosity_with_log() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[command(flatten)]
            verbose: Verbosity<InfoLevel>,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    #[cfg(feature = "tracing")]
    fn verbosity_with_tracing() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[command(flatten)]
            verbose: Verbosity<tracing::ErrorLevel>,
        }

        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
