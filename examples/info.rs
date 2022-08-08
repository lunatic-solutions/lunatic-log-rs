use logger_rs::{info, subscriber::fmt::FmtSubscriber, LevelFilter};

fn main() {
    logger_rs::init(FmtSubscriber::new(LevelFilter::Error));
    info!("Hello, {}", "World");
}
