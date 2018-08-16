extern crate log;
#[macro_use]
extern crate structopt;

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
    /// By default it'll report log level info. Passing `-v` one time also
    /// prints debug, `-vv` enables trace logging.
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
    /// Suppress all log output.
    ///
    /// If you also pass `-v` it'll report errors. Passing `-vv` will also print
    /// warnings, `-vvv` enables info logging, `-vvvv` debug, and `-vvvvv`
    /// trace.
    #[structopt(long = "quiet", short = "q")]
    quiet: bool,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> Option<Level> {
        if self.quiet {
            match self.verbosity {
                0 => None,
                1 => Some(Level::Error),
                2 => Some(Level::Warn),
                3 => Some(Level::Info),
                4 => Some(Level::Debug),
                _ => Some(Level::Trace),
            }
        } else {
            match self.verbosity {
                0 => Some(Level::Info),
                1 => Some(Level::Debug),
                _ => Some(Level::Trace),
            }
        }
    }
}

use std::fmt;

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.verbosity)
    }
}
