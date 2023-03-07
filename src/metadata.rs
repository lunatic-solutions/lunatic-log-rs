//! Metadata containing extra information for each event.

use serde::{Deserialize, Serialize};

use crate::level::Level;

/// Metadata containing extra information for each event including the log level, the module path,
/// file and line number, and node id & process id that the event was created from.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    /// The level of verbosity of the described span.
    level: Level,

    /// The name of the Rust module where the span occurred.
    module_path: String,

    /// The name of the source code file where the span occurred.
    file: String,

    /// The line number in the source code file where the span occurred.
    line: u32,

    /// The node ID where the span occurred.
    node_id: u64,

    /// The process ID where the span occurred.
    process_id: u64,
}

impl Metadata {
    /// Creates a new instance of metadata.
    pub const fn new(
        level: Level,
        module_path: String,
        file: String,
        line: u32,
        node_id: u64,
        process_id: u64,
    ) -> Self {
        Metadata {
            level,
            module_path,
            file,
            line,
            node_id,
            process_id,
        }
    }

    /// The level of verbosity of the described span.
    pub fn level(&self) -> Level {
        self.level
    }

    /// The name of the Rust module where the span occurred.
    pub fn module_path(&self) -> &str {
        &self.module_path
    }

    /// The name of the source code file where the span occurred.
    pub fn file(&self) -> &str {
        &self.file
    }

    /// The line number in the source code file where the span occurred.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// The node ID where the span occurred.
    pub fn node_id(&self) -> u64 {
        self.node_id
    }

    /// The process ID where the span occurred.
    pub fn process_id(&self) -> u64 {
        self.process_id
    }
}

/// Creates an instance of metadata using the current context to populate the fields.
#[macro_export]
macro_rules! metadata {
    ($level:expr) => {{
        extern crate lunatic;

        use std::string::ToString;

        let this_process: lunatic::Process<()> = lunatic::Process::this();
        $crate::metadata::Metadata::new(
            $level,
            module_path!().to_string(),
            file!().to_string(),
            line!(),
            this_process.node_id(),
            this_process.id(),
        )
    }};
}
