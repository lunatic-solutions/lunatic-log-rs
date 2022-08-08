use lunatic::process::{AbstractProcess, MessageHandler, ProcessRef};
use serde::{Deserialize, Serialize};

use crate::subscriber::SubscriberInstance;

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
pub struct Dispatch(pub String);

impl MessageHandler<Dispatch> for LoggingProcess {
    fn handle(state: &mut Self::State, message: Dispatch) {
        state.subscriber.event(&message.0);
    }
}
