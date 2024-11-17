// These re-exports of the log crate make it easy to use this crate without having to depend on the
// log crate directly. See <https://github.com/clap-rs/clap-verbosity-flag/issues/54> for more
// information.
pub use log::{Level, LevelFilter};

use crate::VerbosityFilter;

impl From<VerbosityFilter> for LevelFilter {
    fn from(filter: VerbosityFilter) -> Self {
        match filter {
            VerbosityFilter::Off => LevelFilter::Off,
            VerbosityFilter::Error => LevelFilter::Error,
            VerbosityFilter::Warn => LevelFilter::Warn,
            VerbosityFilter::Info => LevelFilter::Info,
            VerbosityFilter::Debug => LevelFilter::Debug,
            VerbosityFilter::Trace => LevelFilter::Trace,
        }
    }
}

impl From<LevelFilter> for VerbosityFilter {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::Off => Self::Off,
            LevelFilter::Error => Self::Error,
            LevelFilter::Warn => Self::Warn,
            LevelFilter::Info => Self::Info,
            LevelFilter::Debug => Self::Debug,
            LevelFilter::Trace => Self::Trace,
        }
    }
}

impl From<VerbosityFilter> for Option<Level> {
    fn from(filter: VerbosityFilter) -> Self {
        match filter {
            VerbosityFilter::Off => None,
            VerbosityFilter::Error => Some(Level::Error),
            VerbosityFilter::Warn => Some(Level::Warn),
            VerbosityFilter::Info => Some(Level::Info),
            VerbosityFilter::Debug => Some(Level::Debug),
            VerbosityFilter::Trace => Some(Level::Trace),
        }
    }
}

impl From<Option<Level>> for VerbosityFilter {
    fn from(level: Option<Level>) -> Self {
        match level {
            None => Self::Off,
            Some(Level::Error) => Self::Error,
            Some(Level::Warn) => Self::Warn,
            Some(Level::Info) => Self::Info,
            Some(Level::Debug) => Self::Debug,
            Some(Level::Trace) => Self::Trace,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DebugLevel, ErrorLevel, InfoLevel, OffLevel, TraceLevel, Verbosity, WarnLevel};

    #[test]
    fn log_level() {
        let v = Verbosity::<OffLevel>::default();
        assert_eq!(v.log_level(), None);
        assert_eq!(v.log_level_filter(), LevelFilter::Off);

        let v = Verbosity::<ErrorLevel>::default();
        assert_eq!(v.log_level(), Some(Level::Error));
        assert_eq!(v.log_level_filter(), LevelFilter::Error);

        let v = Verbosity::<WarnLevel>::default();
        assert_eq!(v.log_level(), Some(Level::Warn));
        assert_eq!(v.log_level_filter(), LevelFilter::Warn);

        let v = Verbosity::<InfoLevel>::default();
        assert_eq!(v.log_level(), Some(Level::Info));
        assert_eq!(v.log_level_filter(), LevelFilter::Info);

        let v = Verbosity::<DebugLevel>::default();
        assert_eq!(v.log_level(), Some(Level::Debug));
        assert_eq!(v.log_level_filter(), LevelFilter::Debug);

        let v = Verbosity::<TraceLevel>::default();
        assert_eq!(v.log_level(), Some(Level::Trace));
        assert_eq!(v.log_level_filter(), LevelFilter::Trace);
    }
}
