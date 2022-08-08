use serde::{Deserialize, Serialize};

use crate::level::Level;

/// Metadata describing a [span] or [event].
///
/// All spans and events have the following metadata:
/// - A [name], represented as a static string.
/// - A [target], a string that categorizes part of the system where the span
///   or event occurred. The `tracing` macros default to using the module
///   path where the span or event originated as the target, but it may be
///   overridden.
/// - A [verbosity level]. This determines how verbose a given span or event
///   is, and allows enabling or disabling more verbose diagnostics
///   situationally. See the documentation for the [`Level`] type for details.
/// - The names of the [fields] defined by the span or event.
/// - Whether the metadata corresponds to a span or event.
///
/// In addition, the following optional metadata describing the source code
/// location where the span or event originated _may_ be provided:
/// - The [file name]
/// - The [line number]
/// - The [module path]
///
/// Metadata is used by [`Subscriber`]s when filtering spans and events, and it
/// may also be used as part of their data payload.
///
/// When created by the `event!` or `span!` macro, the metadata describing a
/// particular event or span is constructed statically and exists as a single
/// static instance. Thus, the overhead of creating the metadata is
/// _significantly_ lower than that of creating the actual span. Therefore,
/// filtering is based on metadata, rather than on the constructed span.
///
/// ## Equality
///
/// In well-behaved applications, two `Metadata` with equal
/// [callsite identifiers] will be equal in all other ways (i.e., have the same
/// `name`, `target`, etc.). Consequently, in release builds, [`Metadata::eq`]
/// *only* checks that its arguments have equal callsites. However, the equality
/// of `Metadata`'s other fields is checked in debug builds.
///
/// [span]: super::span
/// [event]: super::event
/// [name]: Self::name
/// [target]: Self::target
/// [fields]: Self::fields
/// [verbosity level]: Self::level
/// [file name]: Self::file
/// [line number]: Self::line
/// [module path]: Self::module_path
/// [`Subscriber`]: super::subscriber::Subscriber
/// [callsite identifiers]: Self::callsite
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// The name of the span described by this metadata.
    name: String,

    /// The part of the system that the span that this metadata describes
    /// occurred in.
    target: String,

    /// The level of verbosity of the described span.
    level: Level,

    /// The name of the Rust module where the span occurred, or `None` if this
    /// could not be determined.
    module_path: Option<String>,

    /// The name of the source code file where the span occurred, or `None` if
    /// this could not be determined.
    file: Option<String>,

    /// The line number in the source code file where the span occurred, or
    /// `None` if this could not be determined.
    line: Option<u32>,
}

impl Metadata {
    /// Construct new metadata for a span or event, with a name, target, level, field
    /// names, and optional source code location.
    pub const fn new(
        name: String,
        target: String,
        level: Level,
        module_path: Option<String>,
        file: Option<String>,
        line: Option<u32>,
    ) -> Self {
        Metadata {
            name,
            target,
            level,
            module_path,
            file,
            line,
        }
    }

    /// Returns the level of verbosity of the described span or event.
    pub fn level(&self) -> &Level {
        &self.level
    }

    /// Returns the name of the span.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns a string describing the part of the system where the span or
    /// event that this metadata describes occurred.
    ///
    /// Typically, this is the module path, but alternate targets may be set
    /// when spans or events are constructed.
    pub fn target(&self) -> &String {
        &self.target
    }

    /// Returns the path to the Rust module where the span occurred, or
    /// `None` if the module path is unknown.
    pub fn module_path(&self) -> Option<&String> {
        self.module_path.as_ref()
    }

    /// Returns the name of the source code file where the span
    /// occurred, or `None` if the file is unknown
    pub fn file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    /// Returns the line number in the source code file where the span
    /// occurred, or `None` if the line number is unknown.
    pub fn line(&self) -> Option<u32> {
        self.line
    }
}
