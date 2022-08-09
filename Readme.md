A logging library for lunatic Rust applications.

## Why a new logging library?

Current logging solutions in Rust (log, tracing, ...) depend on global static variables that are
initialized at the start of the app. This doesn't work in lunatic, where each process gets their
own memory space. You would need to re-initialize the logger for each process, and that is not
practical.

`lunatic-log` allows you to run a log subscriber process that collects logging messages from all
running processes.

## How to use `lunatic-log`?

Add it as a dependency:

```toml
lunatic-log = "0.2"
```

In your code:

```rust
use lunatic_log::{info, subscriber::fmt::FmtSubscriber, LevelFilter};

fn main() {
    // Initialize subscriber
    lunatic_log::init(FmtSubscriber::new(LevelFilter::Info).pretty());

    // Log message
    info!("Hello, {}", "World");

    // Wait for events to propagate and display before exiting app
    lunatic::sleep(std::time::Duration::from_millis(50));
}
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
