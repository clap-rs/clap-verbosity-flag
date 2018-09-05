extern crate log;
extern crate structopt;

use log::Level;
#[allow(unused)]
use structopt::clap::{App, Arg, ArgMatches};
use structopt::StructOpt;

#[cfg(not(any(feature = "log-level", feature = "verbosity", feature = "quietness")))]
compile_error!("at least one of the clap-verbosity-flag features needs to be enabled: `log-level`, `verbosity`, `quietness`");

const DEFAULT_VERBOSITY: u8 = 3;

#[allow(unused)]
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
#[derive(Debug, Clone)]
pub struct Verbosity {
    level: Option<Level>,
}

impl StructOpt for Verbosity {
    fn clap<'a, 'b>() -> App<'a, 'b> {
        let app = App::new("clap-verbosity-flag");
        Self::augment_clap(app)
    }
    fn from_clap(matches: &ArgMatches) -> Self {
        let verbosity = DEFAULT_VERBOSITY;
        #[cfg(feature = "quietness")]
        let verbosity = verbosity.saturating_sub(matches.occurrences_of("quietness") as u8);
        #[cfg(feature = "verbosity")]
        let verbosity = verbosity.saturating_add(matches.occurrences_of("verbosity") as u8);
        #[cfg(feature = "log-level")]
        let verbosity = if let Some(level_str) = matches.value_of("level").as_ref() {
            VERBOSITY_LEVELS
                .iter()
                .position(|l| l == level_str)
                .unwrap() as u8
        } else {
            verbosity
        };
        let level = match verbosity {
            0 => None,
            1 => Some(Level::Error),
            2 => Some(Level::Warn),
            3 => Some(Level::Info),
            4 => Some(Level::Debug),
            _ => Some(Level::Trace),
        };
        Verbosity { level: level }
    }
}

impl Verbosity {
    pub fn augment_clap<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        {
            #[cfg(feature = "quietness")]
            let app = app.arg({
                let arg = Arg::with_name("quietness")
                    .group("clap_verbosity_flag")
                    .takes_value(false)
                    .multiple(true)
                    .short("q")
                    .long("quiet");
                #[cfg(feature = "log-level")]
                let arg = arg.help("Pass many times for less log output. see `--log-level`");
                #[cfg(not(feature = "log-level"))]
                let arg = arg.help("Pass many times for less log output.\n\nBy default, it'll report errors, warnings and infos. Passing `-q` one time disables infos, `-qq` disables warnings, `-qqq` disables errors and will print nothing.");
                arg
            });
            #[cfg(feature = "verbosity")]
            let app = app.arg({
                let arg = Arg::with_name("verbosity")
                    .group("clap_verbosity_flag")
                    .takes_value(false)
                    .multiple(true)
                    .short("v")
                    .long("verbose");
                #[cfg(feature = "log-level")]
                let arg = arg.help("Pass many times for more log output. see `--log-level`");
                #[cfg(not(feature = "log-level"))]
                let arg = arg.help("Pass many times for more log output.\n\nBy default, it'll report errors, warnings and infos. Passing `-v` one time also prints debug, `-vv` enables trace logging.");
                arg
            });
            #[cfg(feature = "log-level")]
            let app = app.arg({
                let arg = Arg::with_name("level")
                    .group("clap_verbosity_flag")
                    .takes_value(true)
                    .multiple(false)
                    .long("log-level")
                    .possible_values(VERBOSITY_LEVELS)
                    .hide_possible_values(true);
                #[cfg(not(all(feature = "verbosity", feature = "quietness")))]
                let arg = arg.help("Set log level. [default: info, possible values: quiet, error, warn, info, debug, trace]");
                #[cfg(all(not(feature = "verbosity"), feature = "quietness"))]
                let arg = arg.help("Set log level. [default: info, possible values: quiet, error, warn, info, debug, trace]\n\nAlternatively It\'s possible to use `-q`, `-qq` etc. to decrease the log level.");
                #[cfg(all(feature = "verbosity", not(feature = "quietness")))]
                let arg = arg.help("Set log level. [default: info, possible values: quiet, error, warn, info, debug, trace]\n\nAlternatively It\'s possible to use `-v`, `-vv` to increase the log level.");
                #[cfg(all(feature = "verbosity", feature = "quietness"))]
                let arg = arg.help("Set log level. [default: info, possible values: quiet, error, warn, info, debug, trace]\n\nAlternatively It\'s possible to use `-v`, `-vv` to increase and `-q`, `-qq` etc. to decrease the log level.");
                arg
            });
            app
        }
    }
    pub fn is_subcommand() -> bool {
        false
    }
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> Option<Level> {
        self.level
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
