use lunatic::process::{AbstractProcess, MessageHandler, ProcessRef};
use serde::{Deserialize, Serialize};

use crate::subscriber::Subscriber;

pub struct LoggingProcess<T> {
    subscriber: T,
}

impl<T> AbstractProcess for LoggingProcess<T>
where
    T: Subscriber,
{
    type State = LoggingProcess<T>;
    type Arg = T;

    fn init(_: ProcessRef<Self>, subscriber: T) -> Self::State {
        LoggingProcess { subscriber }
    }

    fn type_name() -> &'static str {
        "LoggingProcess"
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dispatch(pub String);

impl<T> MessageHandler<Dispatch> for LoggingProcess<T>
where
    T: Subscriber,
{
    fn handle(state: &mut Self::State, message: Dispatch) {
        state.subscriber.event(&message.0);
    }
}
