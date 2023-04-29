//! Combine multiple subscribers.

use serde::{Deserialize, Serialize};

use crate::{
    metadata::Metadata,
    subscriber::{spawn_subscriber, Event},
};

use super::{
    init_subscriber, spawn_subscriber_fn, Subscriber, SubscriberAlreadyExistsError,
    SubscriberMessage, SubscriberProcess,
};

/// Combines multiple subscribers into a single subscriber.
///
/// Child subscriber processes are spawned, and each one is notified of incoming events.
#[derive(Default, Serialize, Deserialize)]
pub struct MultipleSubscribers {
    subscribers: Vec<SubscriberProcess>,
}

impl MultipleSubscribers {
    /// Creates an instance of [`MultipleSubscribers`].
    pub fn new() -> Self {
        MultipleSubscribers::default()
    }

    /// Adds a child subscriber which runs in its own process.
    pub fn add_subscriber<S>(mut self, subscriber: S) -> Self
    where
        S: Subscriber + Serialize + for<'de> Deserialize<'de>,
    {
        let process = spawn_subscriber(subscriber);
        self.subscribers.push(process);
        self
    }

    /// Adds a child subscriber which runs in its own process.
    /// This is useful if a subscriber is not serializable.
    pub fn add_subscriber_fn<S>(mut self, subscriber: fn() -> S) -> Self
    where
        S: Subscriber,
    {
        let process = spawn_subscriber_fn(subscriber);
        self.subscribers.push(process);
        self
    }

    /// Initializes as the global subscriber.
    ///
    /// Note, this will cause a panic if a global subscriber has already been initialized.
    /// Use the [`try_init`] to handle this error.
    pub fn init(self) {
        self.try_init().unwrap();
    }

    /// Initializes as the global subscriber, returning an error if a global subscriber has already been initialized.
    pub fn try_init(self) -> Result<SubscriberProcess, SubscriberAlreadyExistsError> {
        init_subscriber(self)
    }
}

impl Subscriber for MultipleSubscribers {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        !self.subscribers.is_empty()
    }

    fn event(&self, event: &Event) {
        for subscriber in &self.subscribers {
            subscriber.send(SubscriberMessage::Event(event.clone()));
        }
    }
}
