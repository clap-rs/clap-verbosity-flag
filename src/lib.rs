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
//! #[cfg(feature="log")]
//! env_logger::Builder::new()
//!     .filter_level(cli.verbose.log_level_filter())
//!     .init();
//! #[cfg(feature="tracing")]
//! tracing_subscriber::fmt()
//!     .with_max_level(cli.verbose.log_level_filter())
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

#[cfg(all(feature = "log", feature = "tracing"))]
compile_error!("feature \"log\" and feature \"tracing\" cannot be enabled at the same time");

#[cfg(feature="log")]
pub use log::{Level, LevelFilter};

#[cfg(feature="tracing")]
pub use tracing::{Level, level_filters::LevelFilter};

/// Logging flags to `#[command(flatten)]` into your CLI
#[derive(clap::Args, Debug, Clone, Default)]
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

    /// Get the log level.
    ///
    /// `None` means all output is disabled.
    pub fn log_level(&self) -> Option<Level> {
        level_enum(self.verbosity())
    }

    /// Get the log level filter.
    pub fn log_level_filter(&self) -> LevelFilter {
        #[cfg(feature="log")]
        return level_enum(self.verbosity())
            .map(|l| l.to_level_filter())
            .unwrap_or(LevelFilter::Off);

        #[cfg(feature="tracing")]
        return level_enum(self.verbosity())
            .map(LevelFilter::from_level)
            .unwrap_or(LevelFilter::OFF);
    }

    /// If the user requested complete silence (i.e. not just no-logging).
    pub fn is_silent(&self) -> bool {
        self.log_level().is_none()
    }

    fn verbosity(&self) -> i8 {
        level_value(L::default()) - (self.quiet as i8) + (self.verbose as i8)
    }
}

#[cfg(feature="log")]
fn level_value(level: Option<Level>) -> i8 {
    match level {
        None => -1,
        Some(Level::Error) => 0,
        Some(Level::Warn) => 1,
        Some(Level::Info) => 2,
        Some(Level::Debug) => 3,
        Some(Level::Trace) => 4,
    }
}

#[cfg(feature="log")]
fn level_enum(verbosity: i8) -> Option<Level> {
    match verbosity {
        std::i8::MIN..=-1 => None,
        0 => Some(Level::Error),
        1 => Some(Level::Warn),
        2 => Some(Level::Info),
        3 => Some(Level::Debug),
        4..=std::i8::MAX => Some(Level::Trace),
    }
}

#[cfg(feature="tracing")]
fn level_value(level: Option<Level>) -> i8 {
    match level {
        None => -1,
        Some(Level::ERROR) => 0,
        Some(Level::WARN) => 1,
        Some(Level::INFO) => 2,
        Some(Level::DEBUG) => 3,
        Some(Level::TRACE) => 4,
    }
}

#[cfg(feature="tracing")]
fn level_enum(verbosity: i8) -> Option<Level> {
    match verbosity {
        std::i8::MIN..=-1 => None,
        0 => Some(Level::ERROR),
        1 => Some(Level::WARN),
        2 => Some(Level::INFO),
        3 => Some(Level::DEBUG),
        4..=std::i8::MAX => Some(Level::TRACE),
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
    fn default() -> Option<Level>;

    fn verbose_help() -> Option<&'static str> {
        Some("Increase logging verbosity")
    }

    fn verbose_long_help() -> Option<&'static str> {
        None
    }

    fn quiet_help() -> Option<&'static str> {
        Some("Decrease logging verbosity")
    }

    fn quiet_long_help() -> Option<&'static str> {
        None
    }
}

/// Default to [`Level::Error`]
#[derive(Copy, Clone, Debug, Default)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    fn default() -> Option<Level> {
        #[cfg(feature="log")]
        return Some(Level::Error);
        #[cfg(feature="tracing")]
        return Some(Level::ERROR);
    }
}

/// Default to [`Level::Warn`]
#[derive(Copy, Clone, Debug, Default)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    fn default() -> Option<Level> {
        #[cfg(feature="log")]
        return Some(Level::Warn);
        #[cfg(feature="tracing")]
        return Some(Level::WARN);
    }
}

/// Default to [`Level::Info`]
#[derive(Copy, Clone, Debug, Default)]
pub struct InfoLevel;

impl LogLevel for InfoLevel {
    fn default() -> Option<Level> {
        #[cfg(feature="log")]
        return Some(Level::Info);
        #[cfg(feature="tracing")]
        return Some(Level::INFO);
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
        Cli::command().debug_assert()
    }
}
