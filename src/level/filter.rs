use std::{
    cmp, fmt, str,
    sync::atomic::{AtomicUsize, Ordering},
};

use lunatic_cached_process::CachedLookup;
use lunatic_message_request::ProcessRequest;
use serde::{Deserialize, Serialize};

use crate::subscriber::{SubscriberMessage, SUBSCRIBER};

use super::{Level, LevelInner};

static MAX_LEVEL: AtomicUsize = AtomicUsize::new(LevelFilter::UNSET_USIZE);

/// A filter comparable to a verbosity [`Level`].
///
/// If a [`Level`] is considered less than a `LevelFilter`, it should be
/// considered enabled; if greater than or equal to the `LevelFilter`,
/// that level is disabled. See [`LevelFilter::current`] for more
/// details.
///
/// Note that this is essentially identical to the `Level` type, but with the
/// addition of an [`OFF`] level that completely disables all trace
/// instrumentation.
///
/// See the documentation for the [`Level`] type to see how `Level`s
/// and `LevelFilter`s interact.
///
/// [`OFF`]: LevelFilter::OFF
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LevelFilter(Option<Level>);

/// Indicates that a string could not be parsed to a valid level.
#[derive(Clone, Debug)]
pub struct ParseLevelFilterError(());

impl From<Level> for LevelFilter {
    #[inline]
    fn from(level: Level) -> Self {
        Self::from_level(level)
    }
}

impl From<Option<Level>> for LevelFilter {
    #[inline]
    fn from(level: Option<Level>) -> Self {
        Self(level)
    }
}

impl From<LevelFilter> for Option<Level> {
    #[inline]
    fn from(filter: LevelFilter) -> Self {
        filter.into_level()
    }
}

impl LevelFilter {
    /// The "off" level.
    ///
    /// Designates that trace instrumentation should be completely disabled.
    pub const OFF: LevelFilter = LevelFilter(None);
    /// The "error" level.
    ///
    /// Designates very serious errors.
    pub const ERROR: LevelFilter = LevelFilter::from_level(Level::ERROR);
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    pub const WARN: LevelFilter = LevelFilter::from_level(Level::WARN);
    /// The "info" level.
    ///
    /// Designates useful information.
    pub const INFO: LevelFilter = LevelFilter::from_level(Level::INFO);
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    pub const DEBUG: LevelFilter = LevelFilter::from_level(Level::DEBUG);
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    pub const TRACE: LevelFilter = LevelFilter(Some(Level::TRACE));

    /// Returns a `LevelFilter` that enables spans and events with verbosity up
    /// to and including `level`.
    pub const fn from_level(level: Level) -> Self {
        Self(Some(level))
    }

    /// Returns the most verbose [`Level`] that this filter accepts, or `None`
    /// if it is [`OFF`].
    ///
    /// [`Level`]: super::Level
    /// [`OFF`]: LevelFilter::OFF
    pub const fn into_level(self) -> Option<Level> {
        self.0
    }

    // These consts are necessary because `as` casts are not allowed as
    // match patterns.
    const ERROR_USIZE: usize = LevelInner::Error as usize;
    const WARN_USIZE: usize = LevelInner::Warn as usize;
    const INFO_USIZE: usize = LevelInner::Info as usize;
    const DEBUG_USIZE: usize = LevelInner::Debug as usize;
    const TRACE_USIZE: usize = LevelInner::Trace as usize;
    // Using the value of the last variant + 1 ensures that we match the value
    // for `Option::None` as selected by the niche optimization for
    // `LevelFilter`. If this is the case, converting a `usize` value into a
    // `LevelFilter` (in `LevelFilter::current`) will be an identity conversion,
    // rather than generating a lookup table.
    const OFF_USIZE: usize = LevelInner::Error as usize + 1;
    const UNSET_USIZE: usize = usize::MAX;

    /// Returns a `LevelFilter` that matches the most verbose [`Level`] that any
    /// currently active [collector] will enable.
    ///
    /// User code should treat this as a *hint*. If a given span or event has a
    /// level *higher* than the returned `LevelFilter`, it will not be enabled.
    /// However, if the level is less than or equal to this value, the span or
    /// event is *not* guaranteed to be enabled; the collector will still
    /// filter each callsite individually.
    ///
    /// Therefore, comparing a given span or event's level to the returned
    /// `LevelFilter` **can** be used for determining if something is
    /// *disabled*, but **should not** be used for determining if something is
    /// *enabled*.
    ///
    /// [`Level`]: super::Level
    /// [collector]: super::Collect
    #[inline(always)]
    pub fn current() -> Self {
        match MAX_LEVEL.load(Ordering::Relaxed) {
            Self::ERROR_USIZE => Self::ERROR,
            Self::WARN_USIZE => Self::WARN,
            Self::INFO_USIZE => Self::INFO,
            Self::DEBUG_USIZE => Self::DEBUG,
            Self::TRACE_USIZE => Self::TRACE,
            Self::OFF_USIZE => Self::OFF,
            Self::UNSET_USIZE => {
                // Load the max level for this process
                match SUBSCRIBER.get() {
                    Some(process) => {
                        let max_level = process
                            .request(SubscriberMessage::MaxLevelHint, ())
                            .unwrap_or(LevelFilter::TRACE);
                        LevelFilter::set_max(max_level);
                        max_level
                    }
                    None => Self::OFF,
                }
            }
            #[cfg(debug_assertions)]
            unknown => unreachable!(
                "/!\\ `LevelFilter` representation seems to have changed! /!\\ \n\
                This is a bug (and it's pretty bad).\n \
                The offending repr was: {:?}",
                unknown,
            ),
            #[cfg(not(debug_assertions))]
            _ => unsafe {
                // Using `unreachable_unchecked` here (rather than
                // `unreachable!()`) is necessary to ensure that rustc generates
                // an identity conversion from integer -> discriminant, rather
                // than generating a lookup table. We want to ensure this
                // function is a single `mov` instruction (on x86) if at all
                // possible, because it is called *every* time a span/event
                // callsite is hit; and it is (potentially) the only code in the
                // hottest path for skipping a majority of callsites when level
                // filtering is in use.
                //
                // safety: This branch is only truly unreachable if we guarantee
                // that no values other than the possible enum discriminants
                // will *ever* be present. The `AtomicUsize` is initialized to
                // the `OFF` value. It is only set by the `set_max` function,
                // which takes a `LevelFilter` as a parameter. This restricts
                // the inputs to `set_max` to the set of valid discriminants.
                // Therefore, **as long as `MAX_VALUE` is only ever set by
                // `set_max`**, this is safe.
                core::hint::unreachable_unchecked()
            },
        }
    }

    pub(crate) fn set_max(LevelFilter(level): LevelFilter) {
        let val = match level {
            Some(Level(level)) => level as usize,
            None => Self::OFF_USIZE,
        };

        // using an AcqRel swap ensures an ordered relationship of writes to the
        // max level.
        MAX_LEVEL.swap(val, Ordering::AcqRel);
    }
}

impl fmt::Display for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LevelFilter::OFF => f.pad("off"),
            LevelFilter::ERROR => f.pad("error"),
            LevelFilter::WARN => f.pad("warn"),
            LevelFilter::INFO => f.pad("info"),
            LevelFilter::DEBUG => f.pad("debug"),
            LevelFilter::TRACE => f.pad("trace"),
        }
    }
}

impl fmt::Debug for LevelFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LevelFilter::OFF => f.pad("LevelFilter::OFF"),
            LevelFilter::ERROR => f.pad("LevelFilter::ERROR"),
            LevelFilter::WARN => f.pad("LevelFilter::WARN"),
            LevelFilter::INFO => f.pad("LevelFilter::INFO"),
            LevelFilter::DEBUG => f.pad("LevelFilter::DEBUG"),
            LevelFilter::TRACE => f.pad("LevelFilter::TRACE"),
        }
    }
}

impl str::FromStr for LevelFilter {
    type Err = ParseLevelFilterError;
    fn from_str(from: &str) -> Result<Self, Self::Err> {
        from.parse::<usize>()
            .ok()
            .and_then(|num| match num {
                0 => Some(LevelFilter::OFF),
                1 => Some(LevelFilter::ERROR),
                2 => Some(LevelFilter::WARN),
                3 => Some(LevelFilter::INFO),
                4 => Some(LevelFilter::DEBUG),
                5 => Some(LevelFilter::TRACE),
                _ => None,
            })
            .or_else(|| match from {
                "" => Some(LevelFilter::ERROR),
                s if s.eq_ignore_ascii_case("error") => Some(LevelFilter::ERROR),
                s if s.eq_ignore_ascii_case("warn") => Some(LevelFilter::WARN),
                s if s.eq_ignore_ascii_case("info") => Some(LevelFilter::INFO),
                s if s.eq_ignore_ascii_case("debug") => Some(LevelFilter::DEBUG),
                s if s.eq_ignore_ascii_case("trace") => Some(LevelFilter::TRACE),
                s if s.eq_ignore_ascii_case("off") => Some(LevelFilter::OFF),
                _ => None,
            })
            .ok_or(ParseLevelFilterError(()))
    }
}

impl fmt::Display for ParseLevelFilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(
            "error parsing level filter: expected one of \"off\", \"error\", \
            \"warn\", \"info\", \"debug\", \"trace\", or a number 0-5",
        )
    }
}

impl std::error::Error for ParseLevelFilterError {}

impl PartialEq<LevelFilter> for Level {
    #[inline(always)]
    fn eq(&self, other: &LevelFilter) -> bool {
        self.0 as usize == filter_as_usize(&other.0)
    }
}

impl PartialOrd<LevelFilter> for Level {
    #[inline(always)]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some(filter_as_usize(&other.0).cmp(&(self.0 as usize)))
    }

    #[inline(always)]
    fn lt(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) < (self.0 as usize)
    }

    #[inline(always)]
    fn le(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) <= (self.0 as usize)
    }

    #[inline(always)]
    fn gt(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) > (self.0 as usize)
    }

    #[inline(always)]
    fn ge(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) >= (self.0 as usize)
    }
}

impl PartialEq<Level> for LevelFilter {
    #[inline(always)]
    fn eq(&self, other: &Level) -> bool {
        filter_as_usize(&self.0) == other.0 as usize
    }
}

impl PartialOrd for LevelFilter {
    #[inline(always)]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline(always)]
    fn lt(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) < filter_as_usize(&self.0)
    }

    #[inline(always)]
    fn le(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) <= filter_as_usize(&self.0)
    }

    #[inline(always)]
    fn gt(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) > filter_as_usize(&self.0)
    }

    #[inline(always)]
    fn ge(&self, other: &LevelFilter) -> bool {
        filter_as_usize(&other.0) >= filter_as_usize(&self.0)
    }
}

impl Ord for LevelFilter {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        filter_as_usize(&other.0).cmp(&filter_as_usize(&self.0))
    }
}

impl PartialOrd<Level> for LevelFilter {
    #[inline(always)]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some((other.0 as usize).cmp(&filter_as_usize(&self.0)))
    }

    #[inline(always)]
    fn lt(&self, other: &Level) -> bool {
        (other.0 as usize) < filter_as_usize(&self.0)
    }

    #[inline(always)]
    fn le(&self, other: &Level) -> bool {
        (other.0 as usize) <= filter_as_usize(&self.0)
    }

    #[inline(always)]
    fn gt(&self, other: &Level) -> bool {
        (other.0 as usize) > filter_as_usize(&self.0)
    }

    #[inline(always)]
    fn ge(&self, other: &Level) -> bool {
        (other.0 as usize) >= filter_as_usize(&self.0)
    }
}

#[inline(always)]
fn filter_as_usize(x: &Option<Level>) -> usize {
    match x {
        Some(Level(f)) => *f as usize,
        None => LevelFilter::OFF_USIZE,
    }
}

pub use static_max_level::STATIC_MAX_LEVEL;

mod static_max_level {
    use super::LevelFilter;

    /// The statically configured maximum trace level.
    ///
    /// See the [module-level documentation] for information on how to configure
    /// this.
    ///
    /// This value is checked by the `event!` and `span!` macros. Code that
    /// manually constructs events or spans via the `Event::record` function or
    /// `Span` constructors should compare the level against this value to
    /// determine if those spans or events are enabled.
    ///
    /// [module-level documentation]: self#compile-time-filters
    pub const STATIC_MAX_LEVEL: LevelFilter = MAX_LEVEL;

    cfg_if::cfg_if! {
        if #[cfg(all(not(debug_assertions), feature = "release_max_level_off"))] {
            const MAX_LEVEL: LevelFilter = LevelFilter::OFF;
        } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_error"))] {
            const MAX_LEVEL: LevelFilter = LevelFilter::ERROR;
        } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_warn"))] {
            const MAX_LEVEL: LevelFilter = LevelFilter::WARN;
        } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_info"))] {
            const MAX_LEVEL: LevelFilter = LevelFilter::INFO;
        } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_debug"))] {
            const MAX_LEVEL: LevelFilter = LevelFilter::DEBUG;
        } else if #[cfg(all(not(debug_assertions), feature = "release_max_level_trace"))] {
            const MAX_LEVEL: LevelFilter = LevelFilter::TRACE;
        } else if #[cfg(feature = "max_level_off")] {
            const MAX_LEVEL: LevelFilter = LevelFilter::OFF;
        } else if #[cfg(feature = "max_level_error")] {
            const MAX_LEVEL: LevelFilter = LevelFilter::ERROR;
        } else if #[cfg(feature = "max_level_warn")] {
            const MAX_LEVEL: LevelFilter = LevelFilter::WARN;
        } else if #[cfg(feature = "max_level_info")] {
            const MAX_LEVEL: LevelFilter = LevelFilter::INFO;
        } else if #[cfg(feature = "max_level_debug")] {
            const MAX_LEVEL: LevelFilter = LevelFilter::DEBUG;
        } else {
            const MAX_LEVEL: LevelFilter = LevelFilter::TRACE;
        }
    }
}
