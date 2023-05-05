//! A logging library for lunatic Rust applications.
//!
//! A [`Subscriber`] is initialized in a [`lunatic::Process`] with [`init`].
//! Logs are emitted to the subscriber when the [`error`], [`warn`], [`info`], [`debug`], [`trace`] macros are used.
//!
//! # Example
//!
//! ```
//! use lunatic_log::{info, subscriber::fmt::FmtSubscriber};
//!
//! // Initialize subscriber
//! init(FmtSubscriber::new(LevelFilter::Info).pretty());
//!
//! // Log info message
//! info!("Hello, {}", "world");
//! ```

#![deny(missing_docs)]

mod level;
#[macro_use]
mod macros;
mod metadata;
pub mod subscriber;

use std::cell::RefCell;

use lunatic::ProcessName;
use lunatic::{process_local, spawn_link, Process};
use serde::{Deserialize, Serialize};
use subscriber::Subscriber;

pub use crate::level::*;
pub use crate::metadata::*;

process_local! {
    static LOGGING_PROCESS: RefCell<LoggingProcess> = RefCell::new(LoggingProcess::NotLookedUp);
}

#[derive(ProcessName)]
struct LoggingProcessID;

enum LoggingProcess {
    NotLookedUp,
    NotPresent,
    Present(Process<Event>),
}

/// Initialize a subscriber to handle log events.
///
/// The subscriber is spawned in a [`lunatic::Process`] and receives log events.
pub fn init(subscriber: impl Subscriber) -> Process<Event> {
    if Process::<Event>::lookup(&LoggingProcessID).is_some() {
        panic!("logger already initialized");
    }

    let process = spawn_subscriber(subscriber);
    process.register(&LoggingProcessID);
    LOGGING_PROCESS.with_borrow_mut(|mut proc| *proc = LoggingProcess::Present(process.clone()));
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
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

// This is an internal function, and it's API is subject to change at any time.
#[doc(hidden)]
pub fn __lookup_logging_process() -> Option<Process<Event>> {
    LOGGING_PROCESS.with(|proc| {
        let proc_ref = proc.borrow();
        match &*proc_ref {
            LoggingProcess::NotLookedUp => {
                std::mem::drop(proc_ref);
                match Process::<Event>::lookup(&LoggingProcessID) {
                    Some(process) => {
                        *proc.borrow_mut() = LoggingProcess::Present(process.clone());
                        Some(process)
                    }
                    None => {
                        *proc.borrow_mut() = LoggingProcess::NotPresent;
                        None
                    }
                }
            }
            LoggingProcess::NotPresent => None,
            LoggingProcess::Present(process) => Some(process.clone()),
        }
    })
}
