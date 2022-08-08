use lunatic::process::{AbstractProcess, MessageHandler, ProcessRef};
use serde::{Deserialize, Serialize};

use crate::{subscriber::SubscriberInstance, Event};

pub struct LoggingProcess {
    subscriber: SubscriberInstance,
}

impl AbstractProcess for LoggingProcess {
    type State = LoggingProcess;
    type Arg = SubscriberInstance;

    fn init(_: ProcessRef<Self>, subscriber: SubscriberInstance) -> Self::State {
        LoggingProcess { subscriber }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dispatch(pub Event);

impl MessageHandler<Dispatch> for LoggingProcess {
    fn handle(state: &mut Self::State, message: Dispatch) {
        if state.subscriber.enabled(message.0.metadata()) {
            state.subscriber.event(message.0);
        }
    }
}
