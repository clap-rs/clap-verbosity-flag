// These re-exports of the log crate make it easy to use this crate without having to depend on the
// log crate directly. See <https://github.com/clap-rs/clap-verbosity-flag/issues/54> for more
// information.
pub use log::{Level, LevelFilter};

use crate::{Filter, LogLevel, Verbosity};

impl From<Filter> for LevelFilter {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => LevelFilter::Off,
            Filter::Error => LevelFilter::Error,
            Filter::Warn => LevelFilter::Warn,
            Filter::Info => LevelFilter::Info,
            Filter::Debug => LevelFilter::Debug,
            Filter::Trace => LevelFilter::Trace,
        }
    }
}

impl From<LevelFilter> for Filter {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Off => Filter::Off,
            LevelFilter::Error => Filter::Error,
            LevelFilter::Warn => Filter::Warn,
            LevelFilter::Info => Filter::Info,
            LevelFilter::Debug => Filter::Debug,
            LevelFilter::Trace => Filter::Trace,
        }
    }
}

impl From<Filter> for Option<Level> {
    fn from(filter: Filter) -> Self {
        match filter {
            Filter::Off => None,
            Filter::Error => Some(Level::Error),
            Filter::Warn => Some(Level::Warn),
            Filter::Info => Some(Level::Info),
            Filter::Debug => Some(Level::Debug),
            Filter::Trace => Some(Level::Trace),
        }
    }
}

impl From<Option<Level>> for Filter {
    fn from(level: Option<Level>) -> Self {
        match level {
            None => Filter::Off,
            Some(Level::Error) => Filter::Error,
            Some(Level::Warn) => Filter::Warn,
            Some(Level::Info) => Filter::Info,
            Some(Level::Debug) => Filter::Debug,
            Some(Level::Trace) => Filter::Trace,
        }
    }
}

impl<L: LogLevel> Verbosity<L> {
    /// Get the log level.
    ///
    /// `None` means all output is disabled.
    pub fn log_level(&self) -> Option<Level> {
        self.filter().into()
    }

    /// Get the log level filter.
    pub fn log_level_filter(&self) -> LevelFilter {
        self.filter().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DebugLevel, ErrorLevel, InfoLevel, OffLevel, TraceLevel, Verbosity, WarnLevel};

    #[test]
    fn log_level_filter() {
        let default: Verbosity = Verbosity::default();
        assert_eq!(default.log_level_filter(), LevelFilter::Error);
        assert_eq!(
            Verbosity::<OffLevel>::default().log_level_filter(),
            LevelFilter::Off
        );
        assert_eq!(
            Verbosity::<ErrorLevel>::default().log_level_filter(),
            LevelFilter::Error
        );
        assert_eq!(
            Verbosity::<WarnLevel>::default().log_level_filter(),
            LevelFilter::Warn
        );
        assert_eq!(
            Verbosity::<InfoLevel>::default().log_level_filter(),
            LevelFilter::Info
        );
        assert_eq!(
            Verbosity::<DebugLevel>::default().log_level_filter(),
            LevelFilter::Debug
        );
        assert_eq!(
            Verbosity::<TraceLevel>::default().log_level_filter(),
            LevelFilter::Trace
        );
    }

    #[test]
    fn log_level() {
        let default: Verbosity = Verbosity::default();
        assert_eq!(default.log_level(), Some(Level::Error));
        assert_eq!(Verbosity::<OffLevel>::default().log_level(), None);
        assert_eq!(
            Verbosity::<ErrorLevel>::default().log_level(),
            Some(Level::Error)
        );
        assert_eq!(
            Verbosity::<WarnLevel>::default().log_level(),
            Some(Level::Warn)
        );
        assert_eq!(
            Verbosity::<InfoLevel>::default().log_level(),
            Some(Level::Info)
        );
        assert_eq!(
            Verbosity::<DebugLevel>::default().log_level(),
            Some(Level::Debug)
        );
        assert_eq!(
            Verbosity::<TraceLevel>::default().log_level(),
            Some(Level::Trace)
        );
    }
}
