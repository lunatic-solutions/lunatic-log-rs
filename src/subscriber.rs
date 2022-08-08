pub mod fmt;
pub mod multiple;

use serde::{de::DeserializeOwned, Serialize};

use crate::{Event, Metadata};

pub trait Subscriber: Serialize + DeserializeOwned {
    fn enabled(&self, metadata: &Metadata) -> bool;
    fn event(&self, event: &Event);
}
