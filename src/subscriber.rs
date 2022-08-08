pub mod fmt;

use serde::{de::DeserializeOwned, Serialize};

pub trait Subscriber: Serialize + DeserializeOwned {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool;

    fn event(&self, event: &str);
}

impl Subscriber for () {
    fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
        unimplemented!("() is not a supported subscriber");
    }

    fn event(&self, _event: &str) {
        unimplemented!("() is not a supported subscriber");
    }
}
