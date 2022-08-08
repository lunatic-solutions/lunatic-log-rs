pub mod process;
pub mod subscriber;

use lunatic::process::{Message, ProcessRef, StartProcess};
use process::LoggingProcess;
use subscriber::Subscriber;

use crate::process::Dispatch;

pub fn init<S>(subscriber: S) -> ProcessRef<LoggingProcess<S>>
where
    S: Subscriber,
{
    LoggingProcess::start_link(subscriber, Some("lunatic::logger"))
}

pub fn info(msg: impl Into<String>) {
    let proc = ProcessRef::<LoggingProcess<()>>::lookup("lunatic::logger").unwrap();
    proc.send(Dispatch(msg.into()))
}
