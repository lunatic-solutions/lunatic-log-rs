pub mod process;
pub mod subscriber;

use lunatic::process::{Message, ProcessRef, StartProcess};
use process::LoggingProcess;
use subscriber::{SubscriberInstance, SubscriberVTable};
use vtable::{HasStaticVTable, VBox};

use crate::process::Dispatch;

pub fn init<S>(subscriber: S) -> ProcessRef<LoggingProcess>
where
    S: HasStaticVTable<SubscriberVTable>,
{
    let subscriber_vbox = VBox::<SubscriberVTable>::new(subscriber);
    LoggingProcess::start(SubscriberInstance(subscriber_vbox), Some("lunatic::logger"))
}

pub fn info(msg: impl Into<String>) {
    let proc = ProcessRef::<LoggingProcess>::lookup("lunatic::logger").unwrap();
    proc.send(Dispatch(msg.into()))
}
