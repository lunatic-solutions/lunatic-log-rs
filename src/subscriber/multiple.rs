use lunatic::Process;
use serde::{Deserialize, Serialize};

use crate::{spawn_subscriber, Event, Metadata};

use super::Subscriber;

#[derive(Default, Serialize, Deserialize)]
pub struct MultipleSubscribers {
    subscribers: Vec<Process<Event>>,
}

impl MultipleSubscribers {
    pub fn new() -> Self {
        MultipleSubscribers::default()
    }

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
        println!("we got an event");
        for subscriber in &self.subscribers {
            subscriber.send(event.clone());
        }
    }
}
