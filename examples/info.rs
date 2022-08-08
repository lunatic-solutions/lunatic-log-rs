use logger_rs::{info, subscriber::fmt::FmtSubscriber};
use vtable::VBox;

fn main() {
    // VBox::new(FmtSubscriber {});
    logger_rs::init(FmtSubscriber {});
    info("fewfe");
}
