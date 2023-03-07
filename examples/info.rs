use lunatic_log::{info, subscriber::fmt::FmtSubscriber, LevelFilter};

fn main() {
    // Initialize subscriber
    FmtSubscriber::new(LevelFilter::INFO).init();

    // Log message
    info!("Hello, {}", "World");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
