use log::Level;

/// Easily add a `--verbose` flag to CLIs using Structopt
///
/// # Examples
///
/// ```rust
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
#[derive(structopt::StructOpt, Debug, Clone)]
pub struct Verbosity {
    /// Pass many times for more log output
    ///
    /// By default, it'll only report errors. Passing `-v` one time also prints
    /// warnings, `-vv` enables info logging, `-vvv` debug, and `-vvvv` trace.
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    /// Pass many times for less log output
    #[structopt(long, short = "q", parse(from_occurrences), conflicts_with = "verbose")]
    quiet: i8,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> Level {
        let verbosity = 0 - self.quiet + self.verbose;

        match verbosity {
            std::i8::MIN..=0 => log::Level::Error,
            1 => log::Level::Warn,
            2 => log::Level::Info,
            3 => log::Level::Debug,
            4..=std::i8::MAX => log::Level::Trace,
        }
    }

    /// If the user requested complete silence (i.e. not just no-logging).
    pub fn is_silent(&self) -> bool {
        let verbosity = 0 - self.quiet + self.verbose;
        verbosity < 0
    }
}

use std::fmt;

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.verbose)
    }
}
