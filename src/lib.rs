mod level;
#[macro_use]
mod macros;
mod metadata;
pub mod process;
pub mod subscriber;

use std::cell::RefCell;

use lunatic::{
    process::{ProcessRef, StartProcess},
    process_local,
};
use process::LoggingProcess;
use serde::{Deserialize, Serialize};
use subscriber::{SubscriberInstance, SubscriberVTable};
use vtable::{HasStaticVTable, VBox};

pub use crate::level::*;
pub use crate::metadata::*;

process_local! {
    static LOGGING_PROCESS: RefCell<Option<ProcessRef<LoggingProcess>>> = RefCell::new(None);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    metadata: Metadata,
    message: String,
}

impl Event {
    pub const fn new(message: String, metadata: Metadata) -> Self {
        Event { metadata, message }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    /// Returns [metadata] describing this `Event`.
    ///
    /// [metadata]: super::Metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

pub fn init<S>(subscriber: S) -> ProcessRef<LoggingProcess>
where
    S: HasStaticVTable<SubscriberVTable>,
{
    tracing::info!("hi");

    let subscriber_vbox = VBox::<SubscriberVTable>::new(subscriber);
    LoggingProcess::start(SubscriberInstance(subscriber_vbox), Some("lunatic::logger"))
}

// pub fn info(msg: impl Into<String>) {
//     let proc = ProcessRef::<LoggingProcess>::lookup("lunatic::logger").unwrap();
//     proc.send(Dispatch(msg.into()))
// }

#[doc(hidden)]
pub fn __lookup_logging_process() -> Option<ProcessRef<LoggingProcess>> {
    LOGGING_PROCESS.with(|proc| {
        if proc.borrow().is_none() {
            return ProcessRef::<LoggingProcess>::lookup("lunatic::logger").map(|process| {
                *proc.borrow_mut() = Some(process.clone());
                process
            });
        }

        proc.borrow().clone()
    })
}
