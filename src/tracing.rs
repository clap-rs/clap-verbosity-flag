// These re-exports of the tracing types make it easy to use this crate without having to depend on
// the tracing crate directly. See <https://github.com/clap-rs/clap-verbosity-flag/issues/54> for
// more information.
pub use tracing_core::{Level, LevelFilter};

use crate::{Filter, LogLevel};

impl From<Filter> for LevelFilter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => LevelFilter::OFF,
            Filter::Error => LevelFilter::ERROR,
            Filter::Warn => LevelFilter::WARN,
            Filter::Info => LevelFilter::INFO,
            Filter::Debug => LevelFilter::DEBUG,
            Filter::Trace => LevelFilter::TRACE,
        }
    }
}

impl From<LevelFilter> for Filter {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::OFF => Filter::Off,
            LevelFilter::ERROR => Filter::Error,
            LevelFilter::WARN => Filter::Warn,
            LevelFilter::INFO => Filter::Info,
            LevelFilter::DEBUG => Filter::Debug,
            LevelFilter::TRACE => Filter::Trace,
        }
    }
}

impl From<Filter> for Option<Level> {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => None,
            Filter::Error => Some(Level::ERROR),
            Filter::Warn => Some(Level::WARN),
            Filter::Info => Some(Level::INFO),
            Filter::Debug => Some(Level::DEBUG),
            Filter::Trace => Some(Level::TRACE),
        }
    }
}

impl From<Option<Level>> for Filter {
    fn from(level: Option<Level>) -> Self {
        match level {
            None => Filter::Off,
            Some(Level::ERROR) => Filter::Error,
            Some(Level::WARN) => Filter::Warn,
            Some(Level::INFO) => Filter::Info,
            Some(Level::DEBUG) => Filter::Debug,
            Some(Level::TRACE) => Filter::Trace,
        }
    }
}

/// Default to [`tracing_core::Level::Error`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct ErrorLevel;

impl LogLevel for ErrorLevel {
    type Level = Level;
    type LevelFilter = LevelFilter;
    fn default() -> Option<Level> {
        Some(Level::ERROR)
    }
}

/// Default to [`tracing_core::Level::Warn`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct WarnLevel;

impl LogLevel for WarnLevel {
    type Level = Level;
    type LevelFilter = LevelFilter;
    fn default() -> Option<Level> {
        Some(Level::WARN)
    }
}

/// Default to [`tracing_core::Level::Info`]
#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, Debug, Default)]
pub struct InfoLevel;

impl LogLevel for InfoLevel {
    type Level = Level;
    type LevelFilter = LevelFilter;
    fn default() -> Option<Level> {
        Some(Level::INFO)
    }
}

#[cfg(test)]
mod tests {
    use crate::Verbosity;

    use super::*;

    #[test]
    fn verbosity_error_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::ERROR), LevelFilter::ERROR),
            (1, 0, Some(Level::WARN), LevelFilter::WARN),
            (2, 0, Some(Level::INFO), LevelFilter::INFO),
            (3, 0, Some(Level::DEBUG), LevelFilter::DEBUG),
            (4, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (5, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (255, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (0, 1, None, LevelFilter::OFF),
            (0, 2, None, LevelFilter::OFF),
            (0, 255, None, LevelFilter::OFF),
            (255, 255, Some(Level::ERROR), LevelFilter::ERROR),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<ErrorLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_warn_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::WARN), LevelFilter::WARN),
            (1, 0, Some(Level::INFO), LevelFilter::INFO),
            (2, 0, Some(Level::DEBUG), LevelFilter::DEBUG),
            (3, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (4, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (255, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (0, 1, Some(Level::ERROR), LevelFilter::ERROR),
            (0, 2, None, LevelFilter::OFF),
            (0, 3, None, LevelFilter::OFF),
            (0, 255, None, LevelFilter::OFF),
            (255, 255, Some(Level::WARN), LevelFilter::WARN),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<WarnLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }

    #[test]
    fn verbosity_info_level() {
        let tests = [
            // verbose, quiet, expected_level, expected_filter
            (0, 0, Some(Level::INFO), LevelFilter::INFO),
            (1, 0, Some(Level::DEBUG), LevelFilter::DEBUG),
            (2, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (3, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (255, 0, Some(Level::TRACE), LevelFilter::TRACE),
            (0, 1, Some(Level::WARN), LevelFilter::WARN),
            (0, 2, Some(Level::ERROR), LevelFilter::ERROR),
            (0, 3, None, LevelFilter::OFF),
            (0, 4, None, LevelFilter::OFF),
            (0, 255, None, LevelFilter::OFF),
            (255, 255, Some(Level::INFO), LevelFilter::INFO),
        ];

        for (verbose, quiet, expected_level, expected_filter) in tests.iter() {
            let v = Verbosity::<InfoLevel>::new(*verbose, *quiet);
            assert_eq!(
                v.log_level(),
                *expected_level,
                "verbose = {verbose}, quiet = {quiet}"
            );
            assert_eq!(
                v.log_level_filter(),
                *expected_filter,
                "verbose = {verbose}, quiet = {quiet}"
            );
        }
    }
}