//! Easily add a `--verbose` flag to CLIs using Structopt
//!
//! # Examples
//!
//! ```rust
//! use clap::Parser;
//! use clap_verbosity_flag::Verbosity;
//!
//! /// Le CLI
//! #[derive(Debug, Parser)]
//! struct Cli {
//!     #[clap(flatten)]
//!     verbose: Verbosity,
//! }
//! #
//! # fn main() {}
//! ```

use log::Level;

#[derive(clap::Args, Debug, Clone)]
pub struct Verbosity {
    /// Pass many times for more log output
    ///
    /// By default, it'll only report errors. Passing `-v` one time also prints
    /// warnings, `-vv` enables info logging, `-vvv` debug, and `-vvvv` trace.
    #[clap(long, short = 'v', parse(from_occurrences))]
    verbose: i8,

    /// Pass many times for less log output
    #[clap(long, short = 'q', parse(from_occurrences), conflicts_with = "verbose")]
    quiet: i8,

    #[clap(skip)]
    default: i8,
}

impl Verbosity {
    /// Create a new verbosity instance by explicitly setting the values
    pub fn new(verbose: i8, quiet: i8, default: i8) -> Verbosity {
        Verbosity {
            verbose,
            quiet,
            default,
        }
    }
    /// Change the default level.
    ///
    /// `None` means all output is disabled.
    pub fn set_default(&mut self, level: Option<Level>) {
        self.default = level_value(level);
    }

    /// Get the log level.
    ///
    /// `None` means all output is disabled.
    pub fn log_level(&self) -> Option<Level> {
        level_enum(self.verbosity())
    }

    /// If the user requested complete silence (i.e. not just no-logging).
    pub fn is_silent(&self) -> bool {
        self.log_level().is_none()
    }

    fn verbosity(&self) -> i8 {
        self.default - self.quiet + self.verbose
    }
}

fn level_value(level: Option<Level>) -> i8 {
    match level {
        None => -1,
        Some(log::Level::Error) => 0,
        Some(log::Level::Warn) => 1,
        Some(log::Level::Info) => 2,
        Some(log::Level::Debug) => 3,
        Some(log::Level::Trace) => 4,
    }
}

fn level_enum(verbosity: i8) -> Option<Level> {
    match verbosity {
        std::i8::MIN..=-1 => None,
        0 => Some(log::Level::Error),
        1 => Some(log::Level::Warn),
        2 => Some(log::Level::Info),
        3 => Some(log::Level::Debug),
        4..=std::i8::MAX => Some(log::Level::Trace),
    }
}

use std::fmt;

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.verbose)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        #[derive(Debug, clap::StructOpt)]
        struct Cli {
            #[clap(flatten)]
            verbose: Verbosity,
        }

        use clap::IntoApp;
        Cli::into_app().debug_assert()
    }
}
