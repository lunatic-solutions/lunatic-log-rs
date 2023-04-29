#![allow(clippy::let_unit_value)]

use lunatic_log::{info, subscriber::fmt::FmtSubscriber, LevelFilter};
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    // Init a subscriber
    FmtSubscriber::new(LevelFilter::TRACE).pretty().init();

    // Create some mock data
    let null = ();
    let bool_t = true;
    let bool_f = false;
    let number = 10.43;
    let string = "Hello, World!";
    let array = vec![1, 2, 3];
    let object = Person {
        name: "John Doe".to_string(),
        age: 23,
    };

    // Log data
    info!(
        null,
        bool_t, bool_f, number, string, array, object, "Additional log message."
    );

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
