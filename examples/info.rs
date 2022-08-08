use logger_rs::{info, subscriber::fmt::FmtSubscriber};

fn main() {
    logger_rs::init(FmtSubscriber {});
    info("Hello, world!");
}
