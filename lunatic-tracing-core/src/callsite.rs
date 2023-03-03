//! Callsites represent the source locations from which spans or events
//! originate.
//!
//! # What Are Callsites?
//!
//! Every span or event in `tracing` is associated with a [`Callsite`]. A
//! callsite is a small `static` value that is responsible for the following:
//!
//! * Storing the span or event's [`Metadata`],
//! * Uniquely [identifying](Identifier) the span or event definition,
//! * Caching the collector's [`Interest`][^1] in that span or event, to avoid
//!   re-evaluating filters,
//! * Storing a [`Registration`] that allows the callsite to be part of a global
//!   list of all callsites in the program.
//!
//! # Registering Callsites
//!
//! When a span or event is recorded for the first time, its callsite
//! [`register`]s itself with the global callsite registry. Registering a
//! callsite calls the [`Collect::register_callsite`][`register_callsite`]
//! method with that callsite's [`Metadata`] on every currently active
//! collector. This serves two primary purposes: informing collectors of the
//! callsite's existence, and performing static filtering.
//!
//! ## Callsite Existence
//!
//! If a [`Collect`] implementation wishes to allocate storage for each
//! unique span/event location in the program, or pre-compute some value
//! that will be used to record that span or event in the future, it can
//! do so in its [`register_callsite`] method.
//!
//! ## Performing Static Filtering
//!
//! The [`register_callsite`] method returns an [`Interest`] value,
//! which indicates that the collector either [always] wishes to record
//! that span or event, [sometimes] wishes to record it based on a
//! dynamic filter evaluation, or [never] wishes to record it.
//!
//! When registering a new callsite, the [`Interest`]s returned by every
//! currently active collector are combined, and the result is stored at
//! each callsite. This way, when the span or event occurs in the
//! future, the cached [`Interest`] value can be checked efficiently
//! to determine if the span or event should be recorded, without
//! needing to perform expensive filtering (i.e. calling the
//! [`Collect::enabled`] method every time a span or event occurs).
//!
//! ### Rebuilding Cached Interest
//!
//! When a new [`Dispatch`] is created (i.e. a new collector becomes
//! active), any previously cached [`Interest`] values are re-evaluated
//! for all callsites in the program. This way, if the new collector
//! will enable a callsite that was not previously enabled, the
//! [`Interest`] in that callsite is updated. Similarly, when a
//! collector is dropped, the interest cache is also re-evaluated, so
//! that any callsites enabled only by that collector are disabled.
//!
//! In addition, the [`rebuild_interest_cache`] function in this module can be
//! used to manually invalidate all cached interest and re-register those
//! callsites. This function is useful in situations where a collector's
//! interest can change, but it does so relatively infrequently. The collector
//! may wish for its interest to be cached most of the time, and return
//! [`Interest::always`][always] or [`Interest::never`][never] in its
//! [`register_callsite`] method, so that its [`Collect::enabled`] method
//! doesn't need to be evaluated every time a span or event is recorded.
//! However, when the configuration changes, the collector can call
//! [`rebuild_interest_cache`] to re-evaluate the entire interest cache with its
//! new configuration. This is a relatively costly operation, but if the
//! configuration changes infrequently, it may be more efficient than calling
//! [`Collect::enabled`] frequently.
//!
//! [^1]: Returned by the [`Collect::register_callsite`][`register_callsite`]
//!     method.
//!
//! [`Metadata`]: crate::metadata::Metadata
//! [`Interest`]: crate::collect::Interest
//! [`Collect`]: crate::collect::Collect
//! [`register_callsite`]: crate::collect::Collect::register_callsite
//! [`Collect::enabled`]: crate::collect::Collect::enabled
//! [always]: crate::collect::Interest::always
//! [sometimes]: crate::collect::Interest::sometimes
//! [never]: crate::collect::Interest::never
//! [`Dispatch`]: crate::dispatch::Dispatch
use lunatic::process::{AbstractProcess, Message, MessageHandler, ProcessRef, StartProcess};
use lunatic_cached_process::{cached_process, CachedLookup, ProcessRefCached};
use serde::{Deserialize, Serialize};

use crate::{
    collect::Interest,
    dispatch::Dispatch,
    metadata::{LevelFilter, Metadata},
};
use core::hash::Hash;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Once,
};

type Callsites = Vec<Callsite>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Callsite {
    interest: AtomicUsize,
    metadata: Metadata,
    // #[serde(skip, default = "default_once")]
    // register: Once,
    // registration: Option<Box<Registration>>,
}

fn default_once() -> Once {
    Once::new()
}

impl Callsite {
    pub const fn new(metadata: Metadata /*, registration: Box<Registration> */) -> Self {
        Self {
            interest: AtomicUsize::new(0xDEADFACED),
            metadata,
            // register: Once::new(),
            // registration: Option::Some(registration),
        }
    }

    /// Registers this callsite with the global callsite registry.
    ///
    /// If the callsite is already registered, this does nothing.
    ///
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This method, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    // #[inline(never)]
    // This only happens once (or if the cached interest value was corrupted).
    // #[cold]
    // pub fn register(&mut self) -> Interest {
    //     self.register
    //         .call_once(|| crate::callsite::register(*self.registration.take().unwrap()));
    //     match self.interest.load(Ordering::Relaxed) {
    //         0 => Interest::never(),
    //         2 => Interest::always(),
    //         _ => Interest::sometimes(),
    //     }
    // }

    /// Returns the callsite's cached Interest, or registers it for the
    /// first time if it has not yet been registered.
    ///
    /// /!\ WARNING: This is *not* a stable API! /!\
    /// This method, and all code contained in the `__macro_support` module, is
    /// a *private* API of `tracing`. It is exposed publicly because it is used
    /// by the `tracing` macros, but it is not part of the stable versioned API.
    /// Breaking changes to this module may occur in small-numbered versions
    /// without warning.
    #[inline]
    pub fn interest(&mut self) -> Interest {
        match self.interest.load(Ordering::Relaxed) {
            0 => Interest::never(),
            1 => Interest::sometimes(),
            2 => Interest::always(),
            _ => Interest::always(), //self.register(),
        }
    }

    pub fn is_enabled(&self, interest: Interest) -> bool {
        interest.is_always()
            || crate::dispatch::get_default(|default| default.enabled(self.metadata.clone()))
    }

    #[inline]
    #[cfg(feature = "log")]
    pub fn disabled_span(&self) -> crate::Span {
        crate::Span::new_disabled(self.meta)
    }

    #[cfg(feature = "log")]
    pub fn log(
        &self,
        logger: &'static dyn log::Log,
        log_meta: log::Metadata<'_>,
        values: &tracing_core::field::ValueSet<'_>,
    ) {
        let meta = self.metadata();
        logger.log(
            &crate::log::Record::builder()
                .file(meta.file())
                .module_path(meta.module_path())
                .line(meta.line())
                .metadata(log_meta)
                .args(format_args!(
                    "{}",
                    crate::log::LogValueSet {
                        values,
                        is_first: true
                    }
                ))
                .build(),
        );
    }

    pub fn set_interest(&self, interest: Interest) {
        let interest = match () {
            _ if interest.is_never() => 0,
            _ if interest.is_always() => 2,
            _ => 1,
        };
        self.interest.store(interest, Ordering::SeqCst);
    }

    #[inline(always)]
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// Uniquely identifies a [`Callsite`]
///
/// Two `Identifier`s are equal if they both refer to the same callsite.
///
/// [`Callsite`]: super::callsite::Callsite
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier(
    /// **Warning**: The fields on this type are currently `pub` because it must
    /// be able to be constructed statically by macros. However, when `const
    /// fn`s are available on stable Rust, this will no longer be necessary.
    /// Thus, these fields are *not* considered stable public API, and they may
    /// change warning. Do not rely on any fields on `Identifier`. When
    /// constructing new `Identifier`s, use the `identify_callsite!` macro or
    /// the `Callsite::id` function instead.
    // TODO: When `Callsite::id` is a const fn, this need no longer be `pub`.
    // #[doc(hidden)]
    // pub &'static dyn Callsite,
    // Use an index into global callsite registry
    pub  usize,
);

/// A registration with the callsite registry.
///
/// Every [`Callsite`] implementation must provide a `&'static Registration`
/// when calling [`register`] to add itself to the global callsite registry.
///
/// See [the documentation on callsite registration][registry-docs] for details
/// on how callsites are registered.
///
/// [`Callsite`]: crate::callsite::Callsite
/// [`register`]: crate::callsite::register
/// [registry-docs]: crate::callsite#registering-callsites
#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    callsite: Callsite,
}

type Dispatchers = Vec<Dispatch>;

const REGISTRY_PROCESS_NAME: &str = "__lunatic-tracing-core-registry";

cached_process! {
    static REGISTRY_PROCESS: ProcessRefCached<Registry> = REGISTRY_PROCESS_NAME;
}

struct Registry {
    callsites: Callsites,
    dispatchers: Dispatchers,
}

impl Registry {
    fn rebuild_interest(&mut self) {
        let mut max_level = LevelFilter::OFF;
        self.dispatchers.retain(|registrar| {
            if registrar.is_set() {
                // If the collector did not provide a max level hint, assume
                // that it may enable every level.
                let level_hint = registrar.max_level_hint().unwrap_or(LevelFilter::TRACE);
                if level_hint > max_level {
                    max_level = level_hint;
                }
                true
            } else {
                false
            }
        });

        self.callsites
            .iter()
            .for_each(|reg| rebuild_callsite_interest(&self.dispatchers, reg));

        LevelFilter::set_max(max_level);
    }

    fn register(&mut self, registration: Registration) {
        rebuild_callsite_interest(&self.dispatchers, &registration.callsite);
        self.callsites.push(registration.callsite);
    }

    fn register_dispatch(&mut self, dispatch: Dispatch) {
        dispatch.on_register_dispatch(dispatch.clone());
        self.dispatchers.push(dispatch);

        self.rebuild_interest();
    }
}

impl AbstractProcess for Registry {
    type Arg = ();
    type State = Self;

    fn init(_: ProcessRef<Self>, _: Self::Arg) -> Self::State {
        Registry {
            callsites: Vec::new(),
            dispatchers: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RebuildInterest;
impl MessageHandler<RebuildInterest> for Registry {
    fn handle(state: &mut Self::State, _: RebuildInterest) {
        state.rebuild_interest()
    }
}

#[derive(Serialize, Deserialize)]
struct Register(Registration);
impl MessageHandler<Register> for Registry {
    fn handle(state: &mut Self::State, Register(registration): Register) {
        state.register(registration)
    }
}

#[derive(Serialize, Deserialize)]
struct RegisterDispatch(Dispatch);
impl MessageHandler<RegisterDispatch> for Registry {
    fn handle(state: &mut Self::State, RegisterDispatch(dispatch): RegisterDispatch) {
        dispatch.on_register_dispatch(dispatch.clone());
        state.dispatchers.push(dispatch);

        state.rebuild_interest();
    }
}

impl Registry {
    pub fn start() -> ProcessRef<Self> {
        <Self as StartProcess<Self>>::start((), Some(REGISTRY_PROCESS_NAME))
    }
}

/// Clear and reregister interest on every [`Callsite`]
///
/// This function is intended for runtime reconfiguration of filters on traces
/// when the filter recalculation is much less frequent than trace events are.
/// The alternative is to have the [`Collect`] that supports runtime
/// reconfiguration of filters always return [`Interest::sometimes()`] so that
/// [`enabled`] is evaluated for every event.
///
/// This function will also re-compute the global maximum level as determined by
/// the [`max_level_hint`] method. If a [`Collect`]
/// implementation changes the value returned by its `max_level_hint`
/// implementation at runtime, then it **must** call this function after that
/// value changes, in order for the change to be reflected.
///
/// See the [documentation on callsite interest caching][cache-docs] for
/// additional information on this function's usage.
///
/// [`max_level_hint`]: crate::collect::Collect::max_level_hint
/// [`Callsite`]: crate::callsite::Callsite
/// [`enabled`]: crate::collect::Collect::enabled
/// [`Interest::sometimes()`]: crate::collect::Interest::sometimes
/// [`Collect`]: crate::collect::Collect
/// [cache-docs]: crate::callsite#rebuilding-cached-interest
pub fn rebuild_interest_cache() {
    if let Some(registry) = REGISTRY_PROCESS.get() {
        registry.send(RebuildInterest);
    }
}

/// Register a new [`Callsite`] with the global registry.
///
/// This should be called once per callsite after the callsite has been
/// constructed.
///
/// See the [documentation on callsite registration][reg-docs] for details
/// on the global callsite registry.
///
/// [`Callsite`]: crate::callsite::Callsite
/// [reg-docs]: crate::callsite#registering-callsites
pub fn register(registration: Registration) {
    if let Some(registry) = REGISTRY_PROCESS.get() {
        registry.send(Register(registration));
    }
}

pub(crate) fn register_dispatch(dispatch: Dispatch) {
    if let Some(registry) = REGISTRY_PROCESS.get() {
        registry.send(RegisterDispatch(dispatch));
    }
}

fn rebuild_callsite_interest(dispatchers: &[Dispatch], callsite: &Callsite) {
    let meta = &callsite.metadata;

    // Iterate over the collectors in the registry, and — if they are
    // active — register the callsite with them.
    let mut interests = dispatchers
        .iter()
        .map(|registrar| registrar.register_callsite(meta.clone())); // TODO: Can we remove this clone?

    // Use the first collector's `Interest` as the base value.
    let interest = if let Some(interest) = interests.next() {
        // Combine all remaining `Interest`s.
        interests.fold(interest, Interest::and)
    } else {
        // If nobody was interested in this thing, just return `never`.
        Interest::never()
    };

    callsite.set_interest(interest)
}

// ===== impl Registration =====

impl Registration {
    /// Construct a new `Registration` from some `&'static dyn Callsite`
    pub const fn new(callsite: Callsite) -> Self {
        Registration { callsite }
    }
}
