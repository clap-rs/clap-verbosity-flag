extern crate log;
#[macro_use]
extern crate structopt;

use log::Level;

const DEFAULT_VERBOSITY: u8 = 3;

#[cfg(feature = "log-level")]
const VERBOSITY_LEVELS: &'static [&'static str] =
    &["quiet", "error", "warn", "info", "debug", "trace"];

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
    /// Pass many times for less log output. see `--log-level`
    #[cfg(feature = "log-level")]
    #[structopt(
        short = "q", long = "quiet", group = "clap_verbosity_flag", parse(from_occurrences)
    )]
    quietness: u8,

    /// Pass many times for more log output. see `--log-level`
    #[cfg(feature = "log-level")]
    #[structopt(
        short = "v", long = "verbose", group = "clap_verbosity_flag", parse(from_occurrences)
    )]
    verbosity: u8,

    /// Pass many times for less log output
    ///
    /// By default, it'll report errors, warnings and infos.
    /// Passing `-q` one time disables infos, `-qq` disables warnings,
    /// `-qqq` disables errors and will print nothing,
    #[cfg(not(feature = "log-level"))]
    #[structopt(
        short = "q", long = "quiet", group = "clap_verbosity_flag", parse(from_occurrences)
    )]
    quietness: u8,

    /// Pass many times for more log output
    ///
    /// By default, it'll report errors, warnings and infos.
    /// Passing `-v` one time also prints debug, `-vv` enables trace logging.
    #[cfg(not(feature = "log-level"))]
    #[structopt(
        short = "v", long = "verbose", group = "clap_verbosity_flag", parse(from_occurrences)
    )]
    verbosity: u8,

    /// Set log level. By default it is info [possible values: quiet, error,
    /// warn, info, debug, trace]
    ///
    /// Alternatively It's possible to use `-v`, `-vv` to increase and `-q`,
    /// `-qq` etc. to decrease the log level.
    #[cfg(feature = "log-level")]
    #[structopt(
        long = "log-level",
        group = "clap_verbosity_flag",
        raw(possible_values = "VERBOSITY_LEVELS", hide_possible_values = "true")
    )]
    level: Option<String>,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> Option<Level> {
        #[cfg(feature = "log-level")]
        let verbosity = if let Some(level) = self.level.as_ref() {
            VERBOSITY_LEVELS.iter().position(|l| l == level).unwrap() as u8
        } else {
            self.verbosity_quietness()
        };
        #[cfg(not(feature = "log-level"))]
        let verbosity = self.verbosity_quietness();

        match verbosity {
            0 => None,
            1 => Some(Level::Error),
            2 => Some(Level::Warn),
            3 => Some(Level::Info),
            4 => Some(Level::Debug),
            _ => Some(Level::Trace),
        }
    }

    fn verbosity_quietness(&self) -> u8 {
        (DEFAULT_VERBOSITY + self.verbosity).saturating_sub(self.quietness)
    }
}

use std::fmt;

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let level = match self.log_level() {
            None => 0,
            Some(Level::Error) => 1,
            Some(Level::Warn) => 2,
            Some(Level::Info) => 3,
            Some(Level::Debug) => 4,
            Some(Level::Trace) => 5,
        };
        write!(f, "{}", level)
    }
}
