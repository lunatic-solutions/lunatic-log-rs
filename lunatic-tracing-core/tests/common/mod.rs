use lunatic_tracing_core::{collect::Collect, metadata::Metadata, span, Event};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestCollectorA;
impl Collect for TestCollectorA {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }
    fn new_span(&self, _: &span::Attributes) -> span::Id {
        span::Id::from_u64(1)
    }
    fn record(&self, _: &span::Id, _: &span::Record) {}
    fn record_follows_from(&self, _: &span::Id, _: &span::Id) {}
    fn event(&self, _: &Event) {}
    fn enter(&self, _: &span::Id) {}
    fn exit(&self, _: &span::Id) {}
    fn current_span(&self) -> span::Current {
        span::Current::unknown()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestCollectorB;
impl Collect for TestCollectorB {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }
    fn new_span(&self, _: &span::Attributes) -> span::Id {
        span::Id::from_u64(1)
    }
    fn record(&self, _: &span::Id, _: &span::Record) {}
    fn record_follows_from(&self, _: &span::Id, _: &span::Id) {}
    fn event(&self, _: &Event) {}
    fn enter(&self, _: &span::Id) {}
    fn exit(&self, _: &span::Id) {}
    fn current_span(&self) -> span::Current {
        span::Current::unknown()
    }
}
