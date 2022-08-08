use logger_rs::{info, subscriber::fmt::FmtSubscriber, LevelFilter};

fn main() {
    // Initialize subscriber
    logger_rs::init(FmtSubscriber::new(LevelFilter::Warn));

    // Log message
    info!("Hello, {}", "World");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
