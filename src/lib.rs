//! A logging library for lunatic Rust applications, inspired by [tracing](https://crates.io/crates/tracing).
//!
//! A global [`Subscriber`] is initialized in its own [`lunatic::Process`] with [`subscriber::init_subscriber`].
//! Logs are emitted to the subscriber as an [`Event`](subscriber::Event) when the [`error`], [`warn`], [`info`], [`debug`], [`trace`] macros are used.
//!
//! # Example
//!
//! ```
//! use lunatic_log::{info, subscriber::fmt::FmtSubscriber};
//!
//! // Initialize subscriber
//! FmtSubscriber::new(LevelFilter::Info).pretty().init();
//!
//! // Log info message
//! info!("Hello, {}", "world");
//! ```

#![deny(missing_docs)]

pub mod level;
pub mod metadata;
pub mod subscriber;

#[macro_use]
mod macros;

pub use level::{Level, LevelFilter};
