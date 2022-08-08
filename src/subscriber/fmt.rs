use lunatic::process::{AbstractProcess, ProcessRef};
use serde::{Deserialize, Serialize};
use tracing::metadata::LevelFilter;

use crate::SubscriberVTable_static;

use super::Subscriber;

pub struct FmtSubscriber {
    // level_filter: LevelFilter,
}

impl Subscriber for FmtSubscriber {
    fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn event(&self, event: &str) {
        println!("{event}");
    }
}

SubscriberVTable_static!(static FMT_SUBSCRIBER_VT for FmtSubscriber);

// impl AbstractProcess for FmtSubscriber {
//     type Arg = LevelFilterWrapper;
//     type State = FmtSubscriber;

//     fn init(this: ProcessRef<Self>, arg: Self::Arg) -> Self::State {
//         todo!()
//     }
// }

// struct LevelFilterWrapper(LevelFilter);

// impl Serialize for LevelFilterWrapper {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let level_num: Option<u8> = if self.0 == LevelFilter::TRACE {
//             Some(0)
//         } else if self.0 == LevelFilter::DEBUG {
//             Some(1)
//         } else if self.0 == LevelFilter::INFO {
//             Some(2)
//         } else if self.0 == LevelFilter::WARN {
//             Some(3)
//         } else if self.0 == LevelFilter::ERROR {
//             Some(4)
//         } else {
//             None
//         };

//         match level_num {
//             Some(num) => serializer.serialize_some(&num),
//             None => serializer.serialize_none(),
//         }
//     }
// }

// impl<'de> Deserialize<'de> for LevelFilterWrapper {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let level_num = <Option<u8>>::deserialize(deserializer)?;
//         let level_filter = match level_num {
//             Some(num) if num == 0 => LevelFilter::TRACE,
//             Some(num) if num == 1 => LevelFilter::DEBUG,
//             Some(num) if num == 2 => LevelFilter::INFO,
//             Some(num) if num == 3 => LevelFilter::WARN,
//             Some(num) if num == 4 => LevelFilter::ERROR,
//             _ => LevelFilter::OFF,
//         };
//         Ok(LevelFilterWrapper(level_filter))
//     }
// }
