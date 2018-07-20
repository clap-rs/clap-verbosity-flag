extern crate env_logger;
extern crate failure;
extern crate log;
#[macro_use]
extern crate structopt;

use env_logger::Builder as LoggerBuilder;
use failure::Error;
use log::Level;

/// Easily add a `--verbose` flag to CLIs using Structopt
///
/// # Examples
///
/// ```rust
/// extern crate clap_verbosity_flag;
/// #[macro_use] extern crate structopt;
///
/// use structopt::StructOpt;
/// use clap_verbosity_flag::Verbosity;
///
/// /// Le CLI
/// #[derive(Debug, StructOpt)]
/// struct Cli {
///     #[structopt(flatten)]
///     verbose: Verbosity,
/// }
/// #
/// # fn main() {}
/// ```
#[derive(StructOpt, Debug, Clone)]
pub struct Verbosity {
    /// Pass many times for more log output
    ///
    /// By default, it'll only report errors. Passing `-v` one time also prints
    /// warnings, `-vv` enables info logging, `-vvv` debug, and `-vvvv` trace.
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> Level {
        match self.verbosity {
            0 => Level::Error,
            1 => Level::Warn,
            2 => Level::Info,
            3 => Level::Debug,
            _ => Level::Trace,
        }
    }

    /// Initialize `env_logger` and set the log level for the given package.
    ///
    /// All other modules default to printing warnings.
    pub fn setup_env_logger(&self, own_pkg_name: &str) -> Result<(), Error> {
        let level_filter = self.log_level().to_level_filter();
        LoggerBuilder::new()
            .filter(Some(&own_pkg_name.replace("-", "_")), level_filter)
            .filter(None, Level::Warn.to_level_filter())
            .try_init()?;
        Ok(())
    }
}

use std::fmt;

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verbosity)
    }
}
