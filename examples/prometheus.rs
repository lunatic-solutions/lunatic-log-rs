use lunatic_log::{
    info,
    subscriber::{self, opentelemetry::OpenTelemetrySubscriber},
    LevelFilter,
};
use opentelemetry::global;

fn main() {
    // Initialize subscriber
    subscriber::init_subscriber_fn(|| {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .install_simple()
            .unwrap();

        OpenTelemetrySubscriber::new(tracer, LevelFilter::TRACE)
    })
    .unwrap();

    // Log message
    info!("Hello, {}", "World");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(500));
}
