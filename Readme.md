A logging library for lunatic Rust applications.

## Problem

Current logging solutions in Rust (log, tracing, ...) depend on global static variables that are
initialized at the start of the app. E.g.

```rust
// tracing
tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
// env_logger
Builder::new()
        .parse(&env::var("MY_APP_LOG").unwrap_or_default())
        .init();
// ...
```

This doesn't work in lunatic, where each process gets their own memory space. You would need to
re-initialize the logger for each process, and that is not practical.

## Solution

Have a "logger" process registered under a well-defined (e.g. lunatic::logger`) name and for every:

```rust
log::info!("informational message");
log::warn!("warning message");
log::error!("this is an error {}", "message");
```

look up the name and send a message to it.

## Implementation suggestions

### 1. Swapable subscriber

The well known process should just define a message interface. This means that we can provide a
simple implementation that writes to standard out:

```rust
logger:init();
```

But the community could also provide their own implementations too:

```rust
open_telemetry_logger:init();
```

### 2. Tracing

We could support in-process tracing similar to: https://crates.io/crates/tracing. Depending on the
lifetime of the span.

### 3. Supervision

The subscriber should be an `AbstractProcess` so that it can be plugged into a supervisor?

### 4. Lookup optimization

Having one subscriber for all processes could become a bottleneck. Looking it up could consist of
two parts:

1. Look up a well-known name (e.g. `lunatic::logger`)
2. Ask it for a subscriber

That way the well known process could maintain a pool of subscribers that is handed out to others.

Once it's looked up, we could save the subscriber process in a process local variable so that we
don't need to look it up each time we do:
```rust
log::info!("informational message");
```