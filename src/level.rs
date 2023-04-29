//! Trace verbosity level filtering.

mod filter;

use std::{cmp, fmt, str};

pub use filter::*;
use serde::{Deserialize, Serialize};

/// Describes the level of verbosity of a span or event.
///
/// # Comparing Levels
///
/// `Level` implements the [`PartialOrd`] and [`Ord`] traits, allowing two
/// `Level`s to be compared to determine which is considered more or less
/// verbose. Levels which are more verbose are considered "greater than" levels
/// which are less verbose, with [`Level::ERROR`] considered the lowest, and
/// [`Level::TRACE`] considered the highest.
///
/// For example:
/// ```
/// use tracing_core::Level;
///
/// assert!(Level::TRACE > Level::DEBUG);
/// assert!(Level::ERROR < Level::WARN);
/// assert!(Level::INFO <= Level::DEBUG);
/// assert_eq!(Level::TRACE, Level::TRACE);
/// ```
///
/// # Filtering
///
/// `Level`s are typically used to implement filtering that determines which
/// spans and events are enabled. Depending on the use case, more or less
/// verbose diagnostics may be desired. For example, when running in
/// development, [`DEBUG`]-level traces may be enabled by default. When running in
/// production, only [`INFO`]-level and lower traces might be enabled. Libraries
/// may include very verbose diagnostics at the [`DEBUG`] and/or [`TRACE`] levels.
/// Applications using those libraries typically chose to ignore those traces. However, when
/// debugging an issue involving said libraries, it may be useful to temporarily
/// enable the more verbose traces.
///
/// The [`LevelFilter`] type is provided to enable filtering traces by
/// verbosity. `Level`s can be compared against [`LevelFilter`]s, and
/// [`LevelFilter`] has a variant for each `Level`, which compares analogously
/// to that level. In addition, [`LevelFilter`] adds a [`LevelFilter::OFF`]
/// variant, which is considered "less verbose" than every other `Level`. This is
/// intended to allow filters to completely disable tracing in a particular context.
///
/// For example:
/// ```
/// use tracing_core::{Level, LevelFilter};
///
/// assert!(LevelFilter::OFF < Level::TRACE);
/// assert!(LevelFilter::TRACE > Level::DEBUG);
/// assert!(LevelFilter::ERROR < Level::WARN);
/// assert!(LevelFilter::INFO <= Level::DEBUG);
/// assert!(LevelFilter::INFO >= Level::INFO);
/// ```
///
/// ## Examples
///
/// Below is a simple example of how a [collector] could implement filtering through
/// a [`LevelFilter`]. When a span or event is recorded, the [`Collect::enabled`] method
/// compares the span or event's `Level` against the configured [`LevelFilter`].
/// The optional [`Collect::max_level_hint`] method can also be implemented to  allow spans
/// and events above a maximum verbosity level to be skipped more efficiently,
/// often improving performance in short-lived programs.
///
/// ```
/// use tracing_core::{span, Event, Level, LevelFilter, Collect, Metadata};
/// # use tracing_core::span::{Id, Record, Current};
///
/// #[derive(Debug)]
/// pub struct MyCollector {
///     /// The most verbose level that this collector will enable.
///     max_level: LevelFilter,
///
///     // ...
/// }
///
/// impl MyCollector {
///     /// Returns a new `MyCollector` which will record spans and events up to
///     /// `max_level`.
///     pub fn with_max_level(max_level: LevelFilter) -> Self {
///         Self {
///             max_level,
///             // ...
///         }
///     }
/// }
/// impl Collect for MyCollector {
///     fn enabled(&self, meta: &Metadata<'_>) -> bool {
///         // A span or event is enabled if it is at or below the configured
///         // maximum level.
///         meta.level() <= &self.max_level
///     }
///
///     // This optional method returns the most verbose level that this
///     // collector will enable. Although implementing this method is not
///     // *required*, it permits additional optimizations when it is provided,
///     // allowing spans and events above the max level to be skipped
///     // more efficiently.
///     fn max_level_hint(&self) -> Option<LevelFilter> {
///         Some(self.max_level)
///     }
///
///     // Implement the rest of the collector...
///     fn new_span(&self, span: &span::Attributes<'_>) -> span::Id {
///         // ...
///         # drop(span); Id::from_u64(1)
///     }

///     fn event(&self, event: &Event<'_>) {
///         // ...
///         # drop(event);
///     }
///
///     // ...
///     # fn enter(&self, _: &Id) {}
///     # fn exit(&self, _: &Id) {}
///     # fn record(&self, _: &Id, _: &Record<'_>) {}
///     # fn record_follows_from(&self, _: &Id, _: &Id) {}
///     # fn current_span(&self) -> Current { Current::unknown() }
/// }
/// ```
///
/// It is worth noting that the `tracing-subscriber` crate provides [additional
/// APIs][envfilter] for performing more sophisticated filtering, such as
/// enabling different levels based on which module or crate a span or event is
/// recorded in.
///
/// [`DEBUG`]: Level::DEBUG
/// [`INFO`]: Level::INFO
/// [`TRACE`]: Level::TRACE
/// [`Collect::enabled`]: crate::collect::Collect::enabled
/// [`Collect::max_level_hint`]: crate::collect::Collect::max_level_hint
/// [collector]: crate::collect::Collect
/// [envfilter]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Level(LevelInner);

impl Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    pub const ERROR: Level = Level(LevelInner::Error);
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    pub const WARN: Level = Level(LevelInner::Warn);
    /// The "info" level.
    ///
    /// Designates useful information.
    pub const INFO: Level = Level(LevelInner::Info);
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    pub const DEBUG: Level = Level(LevelInner::Debug);
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    pub const TRACE: Level = Level(LevelInner::Trace);

    /// Returns the string representation of the `Level`.
    ///
    /// This returns the same string as the `fmt::Display` implementation.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Level::TRACE => "TRACE",
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARN",
            Level::ERROR => "ERROR",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Level::TRACE => f.pad("TRACE"),
            Level::DEBUG => f.pad("DEBUG"),
            Level::INFO => f.pad("INFO"),
            Level::WARN => f.pad("WARN"),
            Level::ERROR => f.pad("ERROR"),
        }
    }
}

impl std::error::Error for ParseLevelError {}

impl str::FromStr for Level {
    type Err = ParseLevelError;
    fn from_str(s: &str) -> Result<Self, ParseLevelError> {
        s.parse::<usize>()
            .map_err(|_| ParseLevelError { _p: () })
            .and_then(|num| match num {
                1 => Ok(Level::ERROR),
                2 => Ok(Level::WARN),
                3 => Ok(Level::INFO),
                4 => Ok(Level::DEBUG),
                5 => Ok(Level::TRACE),
                _ => Err(ParseLevelError { _p: () }),
            })
            .or_else(|_| match s {
                s if s.eq_ignore_ascii_case("error") => Ok(Level::ERROR),
                s if s.eq_ignore_ascii_case("warn") => Ok(Level::WARN),
                s if s.eq_ignore_ascii_case("info") => Ok(Level::INFO),
                s if s.eq_ignore_ascii_case("debug") => Ok(Level::DEBUG),
                s if s.eq_ignore_ascii_case("trace") => Ok(Level::TRACE),
                _ => Err(ParseLevelError { _p: () }),
            })
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum LevelInner {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = 0,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = 1,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = 2,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn = 3,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 4,
}

/// Returned if parsing a `Level` fails.
#[derive(Debug)]
pub struct ParseLevelError {
    _p: (),
}

impl fmt::Display for ParseLevelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(
            "error parsing level: expected one of \"error\", \"warn\", \
             \"info\", \"debug\", \"trace\", or a number 1-5",
        )
    }
}

impl PartialOrd for Level {
    #[inline(always)]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline(always)]
    fn lt(&self, other: &Level) -> bool {
        (other.0 as usize) < (self.0 as usize)
    }

    #[inline(always)]
    fn le(&self, other: &Level) -> bool {
        (other.0 as usize) <= (self.0 as usize)
    }

    #[inline(always)]
    fn gt(&self, other: &Level) -> bool {
        (other.0 as usize) > (self.0 as usize)
    }

    #[inline(always)]
    fn ge(&self, other: &Level) -> bool {
        (other.0 as usize) >= (self.0 as usize)
    }
}

impl Ord for Level {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (other.0 as usize).cmp(&(self.0 as usize))
    }
}

impl Serialize for LevelInner {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for LevelInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let level = u8::deserialize(deserializer)?;
        match level {
            0 => Ok(LevelInner::Trace),
            1 => Ok(LevelInner::Trace),
            2 => Ok(LevelInner::Trace),
            3 => Ok(LevelInner::Trace),
            4 => Ok(LevelInner::Trace),
            n => Err(<D::Error as serde::de::Error>::invalid_value(
                serde::de::Unexpected::Unsigned(n as u64),
                &"number in range 0..=4",
            )),
        }
    }
}
