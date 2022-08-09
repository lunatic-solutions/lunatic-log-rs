//! A [`Subscriber`] handles log events.
//!
//! It can be used to print to stdout with [`FmtSubscriber`](fmt::FmtSubscriber),
//! but is also capable of handling logs in other ways.

pub mod fmt;
pub mod multiple;

use serde::{de::DeserializeOwned, Serialize};

use crate::{Event, Metadata};

/// A subscriber which handles incoming log [`Event`]s.
///
/// # Example
///
/// ```
/// #[derive(Serialize, Deserialize)]
/// pub struct FmtSubscriber {
///     level_filter: LevelFilter,
/// }
///
/// impl Subscriber for FmtSubscriber {
///     fn enabled(&self, metadata: &Metadata) -> bool {
///         metadata.level() <= &self.level_filter
///     }
///
///     fn event(&self, event: &Event) {
///         println!("Log: {}", event.message());
///     }
/// }
/// ```
pub trait Subscriber: Serialize + DeserializeOwned {
    /// Indicate whether subscriber is enabled given some [`Metadata`].
    fn enabled(&self, metadata: &Metadata) -> bool;

    /// Handle a log [`Event`].
    fn event(&self, event: &Event);
}
