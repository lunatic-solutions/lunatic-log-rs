//! A [`Subscriber`] handles log events.
//!
//! It can be used to print to stdout with [`FmtSubscriber`](fmt::FmtSubscriber),
//! but is also capable of handling logs in other ways such as persisting to a file.

#[cfg(feature = "fmt")]
pub mod fmt;
pub mod multiple;

use std::error;

use lunatic::{serializer, spawn, Process};
use lunatic_cached_process::{cached_process, CachedLookup};
use lunatic_message_request::MessageRequest;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{level::LevelFilter, metadata::Metadata};

const SUBSCRIBER_NAME: &str = "lunatic-tracing-subscriber";

cached_process! {
    pub(crate) static SUBSCRIBER: Process<SubscriberMessage, serializer::Json> = SUBSCRIBER_NAME;
}

/// Type alias for a `Process<SubscriberMessage, serializer::Json>`.
pub type SubscriberProcess = Process<SubscriberMessage, serializer::Json>;

/// The message type for subscriber processes.
#[derive(Serialize, Deserialize)]
pub enum SubscriberMessage {
    /// An event dispatched.
    Event(Event),
    /// A request for the current max level hint.
    MaxLevelHint(MessageRequest<(), Option<LevelFilter>>),
}

/// An event containing a message, data, and metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    message: String,
    data: Map<String, Value>,
    metadata: Metadata,
}

impl Event {
    /// Creates a new event given a message and metadata.
    pub const fn new(message: String, data: Map<String, Value>, metadata: Metadata) -> Self {
        Event {
            message,
            data,
            metadata,
        }
    }

    /// Returns the message string to be logged.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the data.
    pub fn data(&self) -> &Map<String, Value> {
        &self.data
    }

    /// Returns [metadata] describing this `Event`.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// A subscriber which handles incoming log [`Event`]s.
///
/// # Example
///
/// ```
/// #[derive(Serialize, Deserialize)]
/// pub struct FmtSubscriber {
///     level_filter: LevelFilter,
/// }
///
/// impl Subscriber for FmtSubscriber {
///     fn enabled(&self, metadata: &Metadata) -> bool {
///         metadata.level() <= &self.level_filter
///     }
///
///     fn event(&self, event: &Event) {
///         println!("Log: {}", event.message());
///     }
/// }
/// ```
pub trait Subscriber: Serialize + for<'de> Deserialize<'de> {
    /// Indicate whether subscriber is enabled given some [`Metadata`].
    fn enabled(&self, metadata: &Metadata) -> bool;

    /// Handle a log [`Event`].
    fn event(&self, event: &Event);

    /// Returns the highest [verbosity level][level] that this `Subscriber` will
    /// enable, or `None`, if the subscriber does not implement level-based
    /// filtering or chooses not to implement this method.
    ///
    /// If this method returns a [`Level`][level], it will be used as a hint to
    /// determine the most verbose level that will be enabled. This will allow
    /// spans and events which are more verbose than that level to be skipped
    /// more efficiently. Subscribers which perform filtering are strongly
    /// encouraged to provide an implementation of this method.
    fn max_level_hint(&self) -> Option<LevelFilter> {
        None
    }
}

/// An error indicating a global subscriber has already been spawned.
#[derive(Debug)]
pub struct SubscriberAlreadyExistsError;

impl std::fmt::Display for SubscriberAlreadyExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "subscriber already exists")
    }
}

impl error::Error for SubscriberAlreadyExistsError {}

/// Initializes a global subscriber in its own process.
///
/// Only one global subscriber may exist, and calling this function multiple times will return an error.
pub fn init_subscriber(
    subscriber: impl Subscriber,
) -> Result<SubscriberProcess, SubscriberAlreadyExistsError> {
    if SUBSCRIBER.get().is_some() {
        return Err(SubscriberAlreadyExistsError);
    }
    let max_level = subscriber.max_level_hint().unwrap_or(LevelFilter::TRACE);
    let process = spawn_subscriber(subscriber);
    process.register(SUBSCRIBER_NAME);
    SUBSCRIBER.set(process);
    LevelFilter::set_max(max_level);
    Ok(process)
}

/// Spawns a subscriber in its own process.
pub fn spawn_subscriber(subscriber: impl Subscriber) -> SubscriberProcess {
    spawn!(
        |subscriber, mailbox: Mailbox<SubscriberMessage, serializer::Json>| {
            loop {
                let message = mailbox.receive();
                match message {
                    SubscriberMessage::Event(event) => {
                        if subscriber.enabled(&event.metadata) {
                            subscriber.event(&event);
                        }
                    }
                    SubscriberMessage::MaxLevelHint(req) => req.reply(subscriber.max_level_hint()),
                }
            }
        }
    )
}

/// Dispatches an event to the global subscriber, if present.
pub fn dispatch(event: Event) {
    if let Some(subscriber) = SUBSCRIBER.get() {
        subscriber.send(SubscriberMessage::Event(event));
    }
}
