use serde::{Deserialize, Serialize};

use crate::{level::LevelFilter, Event, Metadata};

use super::Subscriber;

#[derive(Serialize, Deserialize)]
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

    fn event(&self, event: &Event) {
        println!("{}", event.message());
    }
}
