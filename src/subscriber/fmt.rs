use crate::{level::LevelFilter, metadata::Metadata, Event, SubscriberVTable_static};

use super::Subscriber;

pub struct FmtSubscriber {
    level_filter: LevelFilter,
}

impl FmtSubscriber {
    pub fn new(level_filter: LevelFilter) -> Self {
        FmtSubscriber { level_filter }
    }
}

impl Subscriber for FmtSubscriber {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= &self.level_filter
    }

    fn event(&self, event: Event) {
        println!("{}", event.message());
    }
}

SubscriberVTable_static!(static FMT_SUBSCRIBER_VT for FmtSubscriber);
