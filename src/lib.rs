extern crate log;
#[macro_use]
extern crate structopt;

use log::Level;

const DEFAULT_VERBOSITY: u8 = 3;
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
    /// Pass many times for more log output
    ///
    /// By default, it'll report errors, warnings and infos.
    /// Passing `-v` one time also prints debug, `-vv` enables trace logging.
    #[structopt(short = "v", group = "clap_verbosity_flag", parse(from_occurrences))]
    verbosity: u8,

    /// Set verbosity
    #[structopt(
        long = "verbosity",
        group = "clap_verbosity_flag",
        raw(possible_values = "VERBOSITY_LEVELS"),
        // parse(from_str = "parse_verbosity")
    )]
    level: Option<String>,

    /// Pass many times for less log output
    ///
    /// By default, it'll report errors, warnings and infos.
    /// Passing `-q` one time disables infos, `-qq` disables warnings,
    /// `-qqq` disables errors and will print nothing,
    #[structopt(short = "q", group = "clap_verbosity_flag", parse(from_occurrences))]
    quietness: u8,

    /// Disables all output
    #[structopt(long = "quiet", group = "clap_verbosity_flag")]
    quiet: bool,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> Option<Level> {
        if self.quiet {
            None
        } else if let Some(level) = self.level.as_ref() {
            match level.as_ref() {
                "quiet" => None,
                "error" => Some(Level::Error),
                "warn" => Some(Level::Warn),
                "info" => Some(Level::Info),
                "debug" => Some(Level::Debug),
                "trace" => Some(Level::Trace),
                _ => unreachable!(),
            }
        } else {
            match (DEFAULT_VERBOSITY + self.verbosity).saturating_sub(self.quietness) {
                0 => None,
                1 => Some(Level::Error),
                2 => Some(Level::Warn),
                3 => Some(Level::Info),
                4 => Some(Level::Debug),
                _ => Some(Level::Trace),
            }
        }
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
