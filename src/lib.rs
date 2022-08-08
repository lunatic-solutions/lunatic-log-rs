mod level;
#[macro_use]
mod macros;
mod metadata;
pub mod subscriber;

use std::cell::RefCell;

use lunatic::{process_local, spawn_link, Process};
use serde::{Deserialize, Serialize};
use subscriber::Subscriber;

pub use crate::level::*;
pub use crate::metadata::*;

process_local! {
    static LOGGING_PROCESS: RefCell<Option<Process<Event>>> = RefCell::new(None);
}

/// Initialize a subscriber to log events.
pub fn init(subscriber: impl Subscriber) -> Process<Event> {
    if Process::<Event>::lookup("lunatic::logger").is_some() {
        panic!("logger already initialized");
    }

    let process = spawn_subscriber(subscriber);
    process.register("lunatic::logger");
    process
}

/// Spawn a subscriber process.
pub fn spawn_subscriber(subscriber: impl Subscriber) -> Process<Event> {
    spawn_link!(|subscriber, mailbox: Mailbox<Event>| {
        loop {
            let event = mailbox.receive();
            if subscriber.enabled(event.metadata()) {
                subscriber.event(&event);
            }
        }
    })
}

/// An event to be logged by a subscriber, storing a message and metadata.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    message: String,
    metadata: Metadata,
}

impl Event {
    /// Creates a new event given a message and metadata.
    pub const fn new(message: String, metadata: Metadata) -> Self {
        Event { metadata, message }
    }

    /// Returns the message string to be logged.
    pub fn message(&self) -> &String {
        &self.message
    }

    /// Returns [metadata] describing this `Event`.
    ///
    /// [metadata]: super::Metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

// This is an internal function, and it's API is subject to change at any time.
#[doc(hidden)]
pub fn __lookup_logging_process() -> Option<Process<Event>> {
    LOGGING_PROCESS.with(|proc| {
        if proc.borrow().is_none() {
            return Process::<Event>::lookup("lunatic::logger").map(|process| {
                *proc.borrow_mut() = Some(process.clone());
                process
            });
        }

        proc.borrow().clone()
    })
}
