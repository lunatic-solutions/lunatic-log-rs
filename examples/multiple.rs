use lunatic_log::{
    info,
    subscriber::{fmt::FmtSubscriber, multiple::MultipleSubscribers},
    LevelFilter,
};

fn main() {
    // Initialize multiple subscribers
    MultipleSubscribers::new()
        .add_subscriber(FmtSubscriber::new(LevelFilter::INFO))
        .add_subscriber(FmtSubscriber::new(LevelFilter::INFO))
        .init();

    // Log message
    info!("Hello, {}", "World");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
