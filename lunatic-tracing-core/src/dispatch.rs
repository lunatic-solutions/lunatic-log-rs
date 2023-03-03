//! Dispatches trace events to [`Collect`]s.
//!
//! The _dispatcher_ is the component of the tracing system which is responsible
//! for forwarding trace data from the instrumentation points that generate it
//! to the collector that collects it.
//!
//! # Using the Trace Dispatcher
//!
//! Every thread in a program using `tracing` has a _default collector_. When
//! events occur, or spans are created, they are dispatched to the thread's
//! current collector.
//!
//! ## Setting the Default Collector
//!
//! By default, the current collector is an empty implementation that does
//! nothing. Trace data provided to this "do nothing" implementation is
//! immediately discarded, and is not available for any purpose.
//!
//! To use another collector implementation, it must be set as the default.
//! There are two methods for doing so: [`with_default`] and
//! [`set_global_default`]. `with_default` sets the default collector for the
//! duration of a scope, while `set_global_default` sets a default collector
//! for the entire process.
//!
//! To use either of these functions, we must first wrap our collector in a
//! [`Dispatch`], a cloneable, type-erased reference to a collector. For
//! example:
//! ```rust
//! # pub struct FooCollector;
//! # use lunatic_tracing_core::{
//! #   dispatch, Event, Metadata,
//! #   span::{Attributes, Current, Id, Record}
//! # };
//! # impl tracing_core::Collect for FooCollector {
//! #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
//! #   fn record(&self, _: &Id, _: &Record) {}
//! #   fn event(&self, _: &Event) {}
//! #   fn record_follows_from(&self, _: &Id, _: &Id) {}
//! #   fn enabled(&self, _: &Metadata) -> bool { false }
//! #   fn enter(&self, _: &Id) {}
//! #   fn exit(&self, _: &Id) {}
//! #   fn current_span(&self) -> Current { Current::unknown() }
//! # }
//! # impl FooCollector { fn new() -> Self { FooCollector } }
//! #
//! use dispatch::Dispatch;
//!
//! #
//! let my_collector = FooCollector::new();
//! #
//! let my_dispatch = Dispatch::new(my_collector);
//! ```
//! Then, we can use [`with_default`] to set our `Dispatch` as the default for
//! the duration of a block:
//! ```rust
//! # pub struct FooCollector;
//! # use lunatic_tracing_core::{
//! #   dispatch, Event, Metadata,
//! #   span::{Attributes, Current, Id, Record}
//! # };
//! # impl tracing_core::Collect for FooCollector {
//! #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
//! #   fn record(&self, _: &Id, _: &Record) {}
//! #   fn event(&self, _: &Event) {}
//! #   fn record_follows_from(&self, _: &Id, _: &Id) {}
//! #   fn enabled(&self, _: &Metadata) -> bool { false }
//! #   fn enter(&self, _: &Id) {}
//! #   fn exit(&self, _: &Id) {}
//! #   fn current_span(&self) -> Current { Current::unknown() }
//! # }
//! # impl FooCollector { fn new() -> Self { FooCollector } }
//! # let _my_collector = FooCollector::new();
//! #
//! # let my_dispatch = dispatch::Dispatch::new(_my_collector);
//! // no default collector
//!
//! #
//! dispatch::with_default(&my_dispatch, || {
//!     // my_collector is the default
//! });
//!
//! // no default collector again
//! ```
//! It's important to note that `with_default` will not propagate the current
//! thread's default collector to any threads spawned within the `with_default`
//! block. To propagate the default collector to new threads, either use
//! `with_default` from the new thread, or use `set_global_default`.
//!
//! As an alternative to `with_default`, we can use [`set_global_default`] to
//! set a `Dispatch` as the default for all threads, for the lifetime of the
//! program. For example:
//! ```rust
//! # pub struct FooCollector;
//! # use lunatic_tracing_core::{
//! #   dispatch, Event, Metadata,
//! #   span::{Attributes, Current, Id, Record}
//! # };
//! # impl tracing_core::Collect for FooCollector {
//! #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
//! #   fn record(&self, _: &Id, _: &Record) {}
//! #   fn event(&self, _: &Event) {}
//! #   fn record_follows_from(&self, _: &Id, _: &Id) {}
//! #   fn enabled(&self, _: &Metadata) -> bool { false }
//! #   fn enter(&self, _: &Id) {}
//! #   fn exit(&self, _: &Id) {}
//! #   fn current_span(&self) -> Current { Current::unknown() }
//! # }
//! # impl FooCollector { fn new() -> Self { FooCollector } }
//! #
//! # let my_collector = FooCollector::new();
//! #
//! # let my_dispatch = dispatch::Dispatch::new(my_collector);
//! // no default collector
//!
//! #
//! dispatch::set_global_default(my_dispatch)
//!     // `set_global_default` will return an error if the global default
//!     // collector has already been set.
//!     .expect("global default was already set!");
//!
//! // `my_collector` is now the default
//! ```
//!
//! <div class="example-wrap" style="display:inline-block">
//! <pre class="ignore" style="white-space:normal;font:inherit;">
//!
//! **Note**: the thread-local scoped dispatcher ([`with_default`]) requires the
//! Rust standard library. `no_std` users should use [`set_global_default`] instead.
//!
//! </pre></div>
//!
//! ## Accessing the Default Collector
//!
//! A thread's current default collector can be accessed using the
//! [`get_default`] function, which executes a closure with a reference to the
//! currently default `Dispatch`. This is used primarily by `tracing`
//! instrumentation.
use crate::{
    collect::{self, Collect},
    span, Event, LevelFilter, Metadata,
};

use core::fmt;

use std::{cell::RefCell, error};

use lunatic::{process_local, serializer, spawn, Process};
use lunatic_cached_process::{CachedLookup, ProcessCached};
use lunatic_message_request::{MessageRequest, ProcessRequest};
use serde::{Deserialize, Serialize};

const GLOBAL_DISPATCH_PROCESS_NAME: &str = "__lunatic-tracing-core-global-dispatch-process";

process_local! {
    pub static GLOBAL_DISPATCHER: GlobalDispatchProcess = GlobalDispatchProcess::new();
}

pub type DispatchProcess = Process<DispatchMessage, serializer::Json>;

#[derive(Debug)]
pub struct GlobalDispatchProcess {
    global: ProcessCached<DispatchMessage, serializer::Json>,
    scoped: RefCell<Option<DispatchProcess>>,
}

impl GlobalDispatchProcess {
    pub fn new() -> Self {
        GlobalDispatchProcess {
            global: ProcessCached::new(GLOBAL_DISPATCH_PROCESS_NAME),
            scoped: RefCell::new(None),
        }
    }
}

impl Default for GlobalDispatchProcess {
    fn default() -> Self {
        GlobalDispatchProcess::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DispatchMessage {
    RegisterCallsite(MessageRequest<Metadata, collect::Interest, serializer::Json>),
    OnRegisterDispatch(Dispatch),
    MaxLevelHint(MessageRequest<(), Option<LevelFilter>, serializer::Json>),
    NewSpan(MessageRequest<span::Attributes, span::Id, serializer::Json>),
    CurrentSpan(MessageRequest<(), span::Current, serializer::Json>),
    Enabled(MessageRequest<Metadata, bool, serializer::Json>),
    Event(Event),
    Enter(span::Id),
    Exit(span::Id),
    CloneSpan(span::Id),
    TryClose(MessageRequest<span::Id, bool, serializer::Json>),
    Record {
        span: span::Id,
        values: span::Record,
    },
    RecordFollowsFrom {
        span: span::Id,
        follows: span::Id,
    },
}

/// `Dispatch` trace data to a [`Collect`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dispatch {
    collector: Option<DispatchProcess>,
}

// /// While this guard is active, additional calls to collector functions on
// /// the default dispatcher will not be able to access the dispatch context.
// /// Dropping the guard will allow the dispatch context to be re-entered.
// struct Entered<'a>(&'a State);

/// A guard that resets the current default dispatcher to the prior
/// default dispatcher when dropped.
#[derive(Debug)]
pub struct DefaultGuard(Option<DispatchProcess>);

/// Sets this dispatch as the default for the duration of a closure.
///
/// The default dispatcher is used when creating a new [span] or
/// [`Event`].
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
/// <strong>Note</strong>: This function required the Rust standard library.
/// <!-- hack: this whitespace makes rustdoc interpret the next line as markdown again -->
///
/// `no_std` users should use [`set_global_default`] instead.
///
/// </pre></div>
///
/// [span]: super::span
/// [`Event`]: super::event::Event
pub fn with_default<T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T {
    // When this guard is dropped, the default dispatcher will be reset to the
    // prior default. Using this (rather than simply resetting after calling
    // `f`) ensures that we always reset to the prior dispatcher even if `f`
    // panics.
    let _guard = set_default(dispatcher);
    f()
}

/// Sets the dispatch as the default dispatch for the duration of the lifetime
/// of the returned DefaultGuard
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: This function required the Rust standard library.
/// `no_std` users should use [`set_global_default`] instead.
///
/// </pre></div>
#[must_use = "Dropping the guard unregisters the dispatcher."]
pub fn set_default(dispatcher: &Dispatch) -> DefaultGuard {
    // When this guard is dropped, the default dispatcher will be reset to the
    // prior default. Using this ensures that we always reset to the prior
    // dispatcher even if the thread calling this function panics.
    let old_process = GLOBAL_DISPATCHER.with(|state| state.scoped.replace(dispatcher.collector));
    DefaultGuard(old_process)
}

/// Sets this dispatch as the global default for the duration of the entire program.
/// Will be used as a fallback if no thread-local dispatch has been set in a thread
/// (using `with_default`.)
///
/// Can only be set once; subsequent attempts to set the global default will fail.
/// Returns `Err` if the global default has already been set.
///
///
/// <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
/// <strong>Warning</strong>: In general, libraries should <em>not</em> call
/// <code>set_global_default()</code>! Doing so will cause conflicts when
/// executables that depend on the library try to set the default collector later.
/// </pre></div>
///
/// [span]: super::span
/// [`Event`]: super::event::Event
pub fn set_global_default(dispatcher: &Dispatch) -> Result<(), SetGlobalDefaultError> {
    GLOBAL_DISPATCHER.with(|state| match state.global.get() {
        Some(_) => Err(SetGlobalDefaultError { _no_construct: () }),
        None => {
            if let Some(collector) = dispatcher.collector {
                collector.register(GLOBAL_DISPATCH_PROCESS_NAME);
                state.global.set(collector);
            }

            Ok(())
        }
    })
}

/// Returns true if a `tracing` dispatcher has ever been set.
///
/// This may be used to completely elide trace points if tracing is not in use
/// at all or has yet to be initialized.
#[doc(hidden)]
#[inline(always)]
pub fn has_been_set() -> bool {
    GLOBAL_DISPATCHER.with(|state| state.global.get().is_some())
}

/// Returned if setting the global dispatcher fails.
pub struct SetGlobalDefaultError {
    _no_construct: (),
}

impl fmt::Debug for SetGlobalDefaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SetGlobalDefaultError")
            .field(&Self::MESSAGE)
            .finish()
    }
}

impl fmt::Display for SetGlobalDefaultError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(Self::MESSAGE)
    }
}

impl error::Error for SetGlobalDefaultError {}

impl SetGlobalDefaultError {
    const MESSAGE: &'static str = "a global default trace dispatcher has already been set";
}

/// Executes a closure with a reference to this thread's current [dispatcher].
///
/// Note that calls to `get_default` should not be nested; if this function is
/// called while inside of another `get_default`, that closure will be provided
/// with `Dispatch::none` rather than the previously set dispatcher.
///
/// [dispatcher]: super::dispatch::Dispatch
#[inline(always)]
pub fn get_default<T, F>(f: F) -> T
where
    F: FnOnce(Dispatch) -> T,
{
    GLOBAL_DISPATCHER.with(
        |state| match state.scoped.borrow().or_else(|| state.global.get()) {
            Some(state) => f(Dispatch {
                collector: Some(state),
            }),
            None => f(Dispatch::none()),
        },
    )
}

/// Executes a closure with a reference to this thread's current [dispatcher].
///
/// Note that calls to `get_default` should not be nested; if this function is
/// called while inside of another `get_default`, that closure will be provided
/// with `Dispatch::none` rather than the previously set dispatcher.
///
/// [dispatcher]: super::dispatcher::Dispatch
#[doc(hidden)]
#[inline(never)]
pub fn get_current<T>(f: impl FnOnce(Dispatch) -> T) -> Option<T> {
    GLOBAL_DISPATCHER.with(|state| {
        state.scoped.borrow().map(|state| {
            f(Dispatch {
                collector: Some(state),
            })
        })
    })
}

#[inline(always)]
pub(crate) fn get_global() -> Dispatch {
    GLOBAL_DISPATCHER
        .with(|state| state.global.get().map(Dispatch::some))
        .unwrap_or_else(Dispatch::none)
}

impl Dispatch {
    #[inline]
    pub fn some(collector: DispatchProcess) -> Self {
        Dispatch {
            collector: Some(collector),
        }
    }

    /// Returns a new `Dispatch` that discards events and spans.
    #[inline]
    pub fn none() -> Self {
        Dispatch { collector: None }
    }

    pub fn is_set(&self) -> bool {
        self.collector.is_some()
    }

    pub fn collector(&self) -> Option<DispatchProcess> {
        self.collector.clone()
    }

    /// Returns a `Dispatch` that forwards to the given [`Collect`].
    ///
    /// [`Collect`]: super::collect::Collect
    pub fn spawn<C>(collector: C) -> Self
    where
        C: Collect + Send + Sync + 'static,
    {
        let me = spawn!(
            |collector, mailbox: Mailbox<DispatchMessage, serializer::Json>| {
                loop {
                    match mailbox.receive() {
                        DispatchMessage::RegisterCallsite(metadata) => {
                            let interest = collector.register_callsite(&metadata);
                            metadata.reply(interest);
                        }
                        DispatchMessage::OnRegisterDispatch(c) => {
                            collector.on_register_dispatch(&c)
                        }
                        DispatchMessage::MaxLevelHint(req) => {
                            req.reply(collector.max_level_hint());
                        }
                        DispatchMessage::NewSpan(span) => {
                            let id = collector.new_span(&span);
                            span.reply(id);
                        }
                        DispatchMessage::CurrentSpan(req) => {
                            let current = collector.current_span();
                            req.reply(current);
                        }
                        DispatchMessage::Enabled(metadata) => {
                            let enabled = collector.enabled(&metadata);
                            metadata.reply(enabled);
                        }
                        DispatchMessage::Event(event) => {
                            if collector.event_enabled(&event) {
                                collector.event(&event);
                            }
                        }
                        DispatchMessage::Enter(id) => collector.enter(&id),
                        DispatchMessage::Exit(id) => collector.exit(&id),
                        DispatchMessage::TryClose(req) => {
                            let (process, id) = req.into_parts();
                            let closed = collector.try_close(id);
                            process.reply(closed);
                        }
                        DispatchMessage::CloneSpan(id) => {
                            collector.clone_span(&id);
                        }
                        DispatchMessage::Record { span, values } => {
                            collector.record(&span, &values);
                        }
                        DispatchMessage::RecordFollowsFrom { span, follows } => {
                            collector.record_follows_from(&span, &follows);
                        }
                    }
                }
            }
        );
        let dispatch = Dispatch::some(me);
        crate::callsite::register_dispatch(dispatch.clone());
        dispatch
    }

    /// Registers a new callsite with this collector, returning whether or not
    /// the collector is interested in being notified about the callsite.
    ///
    /// This calls the [`register_callsite`] function on the [`Collect`]
    /// that this `Dispatch` forwards to.
    ///
    /// [`Collect`]: super::collect::Collect
    /// [`register_callsite`]: super::collect::Collect::register_callsite
    #[inline]
    pub fn register_callsite(&self, metadata: Metadata) -> collect::Interest {
        match self.collector {
            Some(collector) => collector.request(DispatchMessage::RegisterCallsite, metadata),
            None => collect::Interest::never(),
        }
    }

    #[inline]
    pub fn on_register_dispatch(&self, collector: Dispatch) {
        if let Some(c) = self.collector {
            c.send(DispatchMessage::OnRegisterDispatch(collector))
        }
    }

    /// Returns the highest [verbosity level][level] that this [collector] will
    /// enable, or `None`, if the collector does not implement level-based
    /// filtering or chooses not to implement this method.
    ///
    /// This calls the [`max_level_hint`] function on the [`Collect`]
    /// that this `Dispatch` forwards to.
    ///
    /// [level]: super::Level
    /// [collector]: super::collect::Collect
    /// [`Collect`]: super::collect::Collect
    /// [`register_callsite`]: super::collect::Collect::max_level_hint
    // TODO(eliza): consider making this a public API?
    #[inline]
    pub(crate) fn max_level_hint(&self) -> Option<LevelFilter> {
        match self.collector {
            Some(collector) => collector.request(DispatchMessage::MaxLevelHint, ()),
            None => None,
        }
    }

    /// Record the construction of a new span, returning a new [ID] for the
    /// span being constructed.
    ///
    /// This calls the [`new_span`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [ID]: super::span::Id
    /// [`Collect`]: super::collect::Collect
    /// [`new_span`]: super::collect::Collect::new_span
    #[inline]
    pub fn new_span(&self, span: span::Attributes) -> span::Id {
        match self.collector {
            Some(collector) => collector.request(DispatchMessage::NewSpan, span),
            None => span::Id::from_u64(0xDEAD),
        }
    }

    /// Record a set of values on a span.
    ///
    /// This calls the [`record`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [`Collect`]: super::collect::Collect
    /// [`record`]: super::collect::Collect::record
    #[inline]
    pub fn record(&self, span: span::Id, values: span::Record) {
        if let Some(collector) = self.collector {
            collector.send(DispatchMessage::Record { span, values })
        }
    }

    /// Adds an indication that `span` follows from the span with the id
    /// `follows`.
    ///
    /// This calls the [`record_follows_from`] function on the [`Collect`]
    /// that this `Dispatch` forwards to.
    ///
    /// [`Collect`]: super::collect::Collect
    /// [`record_follows_from`]: super::collect::Collect::record_follows_from
    #[inline]
    pub fn record_follows_from(&self, span: span::Id, follows: span::Id) {
        if let Some(collector) = self.collector {
            collector.send(DispatchMessage::RecordFollowsFrom { span, follows })
        }
    }

    /// Returns true if a span with the specified [metadata] would be
    /// recorded.
    ///
    /// This calls the [`enabled`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [metadata]: super::metadata::Metadata
    /// [`Collect`]: super::collect::Collect
    /// [`enabled`]: super::collect::Collect::enabled
    #[inline]
    pub fn enabled(&self, metadata: Metadata) -> bool {
        match self.collector {
            Some(collector) => collector.request(DispatchMessage::Enabled, metadata),
            None => false,
        }
    }

    /// Records that an [`Event`] has occurred.
    ///
    /// This calls the [`event`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [`Event`]: super::event::Event
    /// [`Collect`]: super::collect::Collect
    /// [`event`]: super::collect::Collect::event
    #[inline]
    pub fn event(&self, event: Event) {
        if let Some(collector) = self.collector {
            collector.send(DispatchMessage::Event(event))
        }
    }

    /// Records that a span has been can_enter.
    ///
    /// This calls the [`enter`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [`Collect`]: super::collect::Collect
    /// [`enter`]: super::collect::Collect::enter
    pub fn enter(&self, span: span::Id) {
        if let Some(collector) = self.collector {
            collector.send(DispatchMessage::Enter(span))
        }
    }

    /// Records that a span has been exited.
    ///
    /// This calls the [`exit`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [`Collect`]: super::collect::Collect
    /// [`exit`]: super::collect::Collect::exit
    pub fn exit(&self, span: span::Id) {
        if let Some(collector) = self.collector {
            collector.send(DispatchMessage::Exit(span))
        }
    }

    /// Notifies the [collector] that a [span ID] has been cloned.
    ///
    /// This function must only be called with span IDs that were returned by
    /// this `Dispatch`'s [`new_span`] function. The `tracing` crate upholds
    /// this guarantee and any other libraries implementing instrumentation APIs
    /// must as well.
    ///
    /// This calls the [`clone_span`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [span ID]: super::span::Id
    /// [collector]: super::collect::Collect
    /// [`clone_span`]: super::collect::Collect::clone_span
    /// [`new_span`]: super::collect::Collect::new_span
    #[inline]
    pub fn clone_span(&self, id: span::Id) -> span::Id {
        if let Some(collector) = self.collector {
            collector.send(DispatchMessage::CloneSpan(id.clone()));
        }
        id
    }

    /// Notifies the collector that a [span ID] has been dropped, and returns
    /// `true` if there are now 0 IDs referring to that span.
    ///
    /// This function must only be called with span IDs that were returned by
    /// this `Dispatch`'s [`new_span`] function. The `tracing` crate upholds
    /// this guarantee and any other libraries implementing instrumentation APIs
    /// must as well.
    ///
    /// This calls the [`try_close`] function on the [`Collect`] trait
    /// that this `Dispatch` forwards to.
    ///
    /// [span ID]: super::span::Id
    /// [`Collect`]: super::collect::Collect
    /// [`try_close`]: super::collect::Collect::try_close
    /// [`new_span`]: super::collect::Collect::new_span
    pub fn try_close(&self, id: span::Id) -> bool {
        match self.collector {
            Some(collector) => collector.request(DispatchMessage::TryClose, id),
            None => false,
        }
    }

    /// Returns a type representing this collector's view of the current span.
    ///
    /// This calls the [`current`] function on the [`Collect`] that this
    /// `Dispatch` forwards to.
    ///
    /// [`Collect`]: super::collect::Collect
    /// [`current`]: super::collect::Collect::current_span
    #[inline]
    pub fn current_span(&self) -> span::Current {
        match self.collector {
            Some(collector) => collector.request(DispatchMessage::CurrentSpan, ()),
            None => span::Current::none(),
        }
    }
}

impl Default for Dispatch {
    /// Returns the current default dispatcher
    fn default() -> Self {
        get_default(|default| default.clone())
    }
}

// ===== impl Entered =====

// impl<'a> Entered<'a> {
//     #[inline]
//     fn current(&self) -> RefMut<'a, Dispatch> {
//         let default = self.0.default.borrow_mut();
//         RefMut::map(default, |default| {
//             default.get_or_insert_with(|| get_global().clone())
//         })
//     }
// }

// impl<'a> Drop for Entered<'a> {
//     #[inline]
//     fn drop(&mut self) {
//         self.0.can_enter.set(true);
//     }
// }

// ===== impl DefaultGuard =====

impl Drop for DefaultGuard {
    #[inline]
    fn drop(&mut self) {
        GLOBAL_DISPATCHER.with(|state| {
            *state.scoped.borrow_mut() = self.0.clone();
        });
    }
}
