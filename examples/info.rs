use lunatic_log::{info, subscriber::fmt::FmtSubscriber, LevelFilter};

fn main() {
    // Initialize subscriber
    lunatic_log::init(FmtSubscriber::new(LevelFilter::Info));

    // Log message
    info!("Hello, {}", "World");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
