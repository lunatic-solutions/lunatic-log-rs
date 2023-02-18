//! Combine multiple subscribers.

use lunatic::Process;
use serde::{Deserialize, Serialize};

use crate::{spawn_subscriber, Event, Metadata};

use super::Subscriber;

/// Combines multiple subscribers into a single subscriber.
///
/// Child subscriber processes are spawned, and each one is notified of incoming events.
#[derive(Default, Serialize, Deserialize)]
pub struct MultipleSubscribers {
    subscribers: Vec<Process<Event>>,
}

impl MultipleSubscribers {
    /// Creates an instance of [`MultipleSubscribers`].
    pub fn new() -> Self {
        MultipleSubscribers::default()
    }

    /// Adds a child subscriber which runs in its own process.
    pub fn add_subscriber(mut self, subscriber: impl Subscriber) -> Self {
        let process = spawn_subscriber(subscriber);
        self.subscribers.push(process);
        self
    }
}

impl Subscriber for MultipleSubscribers {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        !self.subscribers.is_empty()
    }

    fn event(&self, event: &Event) {
        for subscriber in &self.subscribers {
            subscriber.send(event.clone());
        }
    }
}
