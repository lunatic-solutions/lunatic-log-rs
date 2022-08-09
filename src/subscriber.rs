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
