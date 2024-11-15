// These re-exports of the tracing types make it easy to use this crate without having to depend on
// the tracing crate directly. See <https://github.com/clap-rs/clap-verbosity-flag/issues/54> for
// more information.
pub use tracing_core::{Level, LevelFilter};

use crate::{Filter, LogLevel, Verbosity};

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

impl<L: LogLevel> Verbosity<L> {
    /// Get the trace level.
    ///
    /// `None` means all output is disabled.
    pub fn tracing_level(&self) -> Option<Level> {
        self.filter().into()
    }

    /// Get the trace level filter.
    pub fn tracing_level_filter(&self) -> LevelFilter {
        self.filter().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DebugLevel, ErrorLevel, InfoLevel, OffLevel, TraceLevel, Verbosity, WarnLevel};

    #[test]
    fn tracing_level_filter() {
        let default: Verbosity = Verbosity::default();
        assert_eq!(default.tracing_level_filter(), LevelFilter::ERROR);
        assert_eq!(
            Verbosity::<OffLevel>::default().tracing_level_filter(),
            LevelFilter::OFF
        );
        assert_eq!(
            Verbosity::<ErrorLevel>::default().tracing_level_filter(),
            LevelFilter::ERROR
        );
        assert_eq!(
            Verbosity::<WarnLevel>::default().tracing_level_filter(),
            LevelFilter::WARN
        );
        assert_eq!(
            Verbosity::<InfoLevel>::default().tracing_level_filter(),
            LevelFilter::INFO
        );
        assert_eq!(
            Verbosity::<DebugLevel>::default().tracing_level_filter(),
            LevelFilter::DEBUG
        );
        assert_eq!(
            Verbosity::<TraceLevel>::default().tracing_level_filter(),
            LevelFilter::TRACE
        );
    }

    #[test]
    fn tracing_level() {
        let default: Verbosity = Verbosity::default();
        assert_eq!(default.tracing_level(), Some(Level::ERROR));
        assert_eq!(Verbosity::<OffLevel>::default().tracing_level(), None);
        assert_eq!(
            Verbosity::<ErrorLevel>::default().tracing_level(),
            Some(Level::ERROR)
        );
        assert_eq!(
            Verbosity::<WarnLevel>::default().tracing_level(),
            Some(Level::WARN)
        );
        assert_eq!(
            Verbosity::<InfoLevel>::default().tracing_level(),
            Some(Level::INFO)
        );
        assert_eq!(
            Verbosity::<DebugLevel>::default().tracing_level(),
            Some(Level::DEBUG)
        );
        assert_eq!(
            Verbosity::<TraceLevel>::default().tracing_level(),
            Some(Level::TRACE)
        );
    }
}
