// /// Constructs a new span.
// ///
// /// See [the top-level documentation][lib] for details on the syntax accepted by
// /// this macro.
// ///
// /// [lib]: crate#using-the-macros
// ///
// /// # Examples
// ///
// /// Creating a new span:
// /// ```
// /// # use tracing::{span, Level};
// /// # fn main() {
// /// let span = span!(Level::TRACE, "my span");
// /// let _enter = span.enter();
// /// // do work inside the span...
// /// # }
// /// ```
// #[macro_export]
// macro_rules! span {
//     (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => {
//         $crate::span!(target: $target, parent: $parent, $lvl, $name,)
//     };
//     (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
//         {
//             use $crate::__macro_support::Callsite as _;
//             static CALLSITE: $crate::__macro_support::MacroCallsite = $crate::callsite2! {
//                 name: $name,
//                 kind: $crate::metadata::Kind::SPAN,
//                 target: $target,
//                 level: $lvl,
//                 fields: $($fields)*
//             };
//             let mut interest = $crate::collect::Interest::never();
//             if $crate::level_enabled!($lvl)
//                 && { interest = CALLSITE.interest(); !interest.is_never() }
//                 && CALLSITE.is_enabled(interest)
//             {
//                 let meta = CALLSITE.metadata();
//                 // span with explicit parent
//                 $crate::Span::child_of(
//                     $parent,
//                     meta,
//                     &$crate::valueset!(meta.fields(), $($fields)*),
//                 )
//             } else {
//                 let span = CALLSITE.disabled_span();
//                 $crate::if_log_enabled! { $lvl, {
//                     span.record_all(&$crate::valueset!(CALLSITE.metadata().fields(), $($fields)*));
//                 }};
//                 span
//             }
//         }
//     };
//     (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
//         {
//             use $crate::__macro_support::{Callsite as _, Registration};
//             static CALLSITE: $crate::__macro_support::MacroCallsite = $crate::callsite2! {
//                 name: $name,
//                 kind: $crate::metadata::Kind::SPAN,
//                 target: $target,
//                 level: $lvl,
//                 fields: $($fields)*
//             };

//             let mut interest = $crate::collect::Interest::never();
//             if $crate::level_enabled!($lvl)
//                 && { interest = CALLSITE.interest(); !interest.is_never() }
//                 && CALLSITE.is_enabled(interest)
//             {
//                 let meta = CALLSITE.metadata();
//                 // span with contextual parent
//                 $crate::Span::new(
//                     meta,
//                     &$crate::valueset!(meta.fields(), $($fields)*),
//                 )
//             } else {
//                 let span = CALLSITE.disabled_span();
//                 $crate::if_log_enabled! { $lvl, {
//                     span.record_all(&$crate::valueset!(CALLSITE.metadata().fields(), $($fields)*));
//                 }};
//                 span
//             }
//         }
//     };
//     (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => {
//         $crate::span!(target: $target, parent: $parent, $lvl, $name,)
//     };
//     (parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             parent: $parent,
//             $lvl,
//             $name,
//             $($fields)*
//         )
//     };
//     (parent: $parent:expr, $lvl:expr, $name:expr) => {
//         $crate::span!(
//             target: module_path!(),
//             parent: $parent,
//             $lvl,
//             $name,
//         )
//     };
//     (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
//         $crate::span!(
//             target: $target,
//             $lvl,
//             $name,
//             $($fields)*
//         )
//     };
//     (target: $target:expr, $lvl:expr, $name:expr) => {
//         $crate::span!(target: $target, $lvl, $name,)
//     };
//     ($lvl:expr, $name:expr, $($fields:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             $lvl,
//             $name,
//             $($fields)*
//         )
//     };
//     ($lvl:expr, $name:expr) => {
//         $crate::span!(
//             target: module_path!(),
//             $lvl,
//             $name,
//         )
//     };
// }

// /// Constructs a span at the trace level.
// ///
// /// [Fields] and [attributes] are set using the same syntax as the [`span!`]
// /// macro.
// ///
// /// See [the top-level documentation][lib] for details on the syntax accepted by
// /// this macro.
// ///
// /// [lib]: crate#using-the-macros
// /// [attributes]: crate#configuring-attributes
// /// [Fields]: crate#recording-fields
// /// [`span!`]: span!
// ///
// /// # Examples
// ///
// /// ```rust
// /// # use tracing::{trace_span, span, Level};
// /// # fn main() {
// /// trace_span!("my_span");
// /// // is equivalent to:
// /// span!(Level::TRACE, "my_span");
// /// # }
// /// ```
// ///
// /// ```rust
// /// # use tracing::{trace_span, span, Level};
// /// # fn main() {
// /// let span = trace_span!("my span");
// /// span.in_scope(|| {
// ///     // do work inside the span...
// /// });
// /// # }
// /// ```
// #[macro_export]
// macro_rules! trace_span {
//     (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             parent: $parent,
//             $crate::Level::TRACE,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, parent: $parent:expr, $name:expr) => {
//         $crate::trace_span!(target: $target, parent: $parent, $name,)
//     };
//     (parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             parent: $parent,
//             $crate::Level::TRACE,
//             $name,
//             $($field)*
//         )
//     };
//     (parent: $parent:expr, $name:expr) => {
//         $crate::trace_span!(parent: $parent, $name,)
//     };
//     (target: $target:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             $crate::Level::TRACE,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, $name:expr) => {
//         $crate::trace_span!(target: $target, $name,)
//     };
//     ($name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             $crate::Level::TRACE,
//             $name,
//             $($field)*
//         )
//     };
//     ($name:expr) => { $crate::trace_span!($name,) };
// }

// /// Constructs a span at the debug level.
// ///
// /// [Fields] and [attributes] are set using the same syntax as the [`span!`]
// /// macro.
// ///
// /// See [the top-level documentation][lib] for details on the syntax accepted by
// /// this macro.
// ///
// /// [lib]: crate#using-the-macros
// /// [attributes]: crate#configuring-attributes
// /// [Fields]: crate#recording-fields
// /// [`span!`]: span!
// ///
// /// # Examples
// ///
// /// ```rust
// /// # use tracing::{debug_span, span, Level};
// /// # fn main() {
// /// debug_span!("my_span");
// /// // is equivalent to:
// /// span!(Level::DEBUG, "my_span");
// /// # }
// /// ```
// ///
// /// ```rust
// /// # use tracing::debug_span;
// /// # fn main() {
// /// let span = debug_span!("my span");
// /// span.in_scope(|| {
// ///     // do work inside the span...
// /// });
// /// # }
// /// ```
// #[macro_export]
// macro_rules! debug_span {
//     (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             parent: $parent,
//             $crate::Level::DEBUG,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, parent: $parent:expr, $name:expr) => {
//         $crate::debug_span!(target: $target, parent: $parent, $name,)
//     };
//     (parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             parent: $parent,
//             $crate::Level::DEBUG,
//             $name,
//             $($field)*
//         )
//     };
//     (parent: $parent:expr, $name:expr) => {
//         $crate::debug_span!(parent: $parent, $name,)
//     };
//     (target: $target:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             $crate::Level::DEBUG,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, $name:expr) => {
//         $crate::debug_span!(target: $target, $name,)
//     };
//     ($name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             $crate::Level::DEBUG,
//             $name,
//             $($field)*
//         )
//     };
//     ($name:expr) => {$crate::debug_span!($name,)};
// }

// /// Constructs a span at the info level.
// ///
// /// [Fields] and [attributes] are set using the same syntax as the [`span!`]
// /// macro.
// ///
// /// See [the top-level documentation][lib] for details on the syntax accepted by
// /// this macro.
// ///
// /// [lib]: crate#using-the-macros
// /// [attributes]: crate#configuring-attributes
// /// [Fields]: crate#recording-fields
// /// [`span!`]: span!
// ///
// /// # Examples
// ///
// /// ```rust
// /// # use tracing::{span, info_span, Level};
// /// # fn main() {
// /// info_span!("my_span");
// /// // is equivalent to:
// /// span!(Level::INFO, "my_span");
// /// # }
// /// ```
// ///
// /// ```rust
// /// # use tracing::info_span;
// /// # fn main() {
// /// let span = info_span!("my span");
// /// span.in_scope(|| {
// ///     // do work inside the span...
// /// });
// /// # }
// /// ```
// #[macro_export]
// macro_rules! info_span {
//     (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             parent: $parent,
//             $crate::Level::INFO,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, parent: $parent:expr, $name:expr) => {
//         $crate::info_span!(target: $target, parent: $parent, $name,)
//     };
//     (parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             parent: $parent,
//             $crate::Level::INFO,
//             $name,
//             $($field)*
//         )
//     };
//     (parent: $parent:expr, $name:expr) => {
//         $crate::info_span!(parent: $parent, $name,)
//     };
//     (target: $target:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             $crate::Level::INFO,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, $name:expr) => {
//         $crate::info_span!(target: $target, $name,)
//     };
//     ($name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             $crate::Level::INFO,
//             $name,
//             $($field)*
//         )
//     };
//     ($name:expr) => {$crate::info_span!($name,)};
// }

// /// Constructs a span at the warn level.
// ///
// /// [Fields] and [attributes] are set using the same syntax as the [`span!`]
// /// macro.
// ///
// /// See [the top-level documentation][lib] for details on the syntax accepted by
// /// this macro.
// ///
// /// [lib]: crate#using-the-macros
// /// [attributes]: crate#configuring-attributes
// /// [Fields]: crate#recording-fields
// /// [`span!`]: span!
// ///
// /// # Examples
// ///
// /// ```rust
// /// # use tracing::{warn_span, span, Level};
// /// # fn main() {
// /// warn_span!("my_span");
// /// // is equivalent to:
// /// span!(Level::WARN, "my_span");
// /// # }
// /// ```
// ///
// /// ```rust
// /// use tracing::warn_span;
// /// # fn main() {
// /// let span = warn_span!("my span");
// /// span.in_scope(|| {
// ///     // do work inside the span...
// /// });
// /// # }
// /// ```
// #[macro_export]
// macro_rules! warn_span {
//     (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             parent: $parent,
//             $crate::Level::WARN,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, parent: $parent:expr, $name:expr) => {
//         $crate::warn_span!(target: $target, parent: $parent, $name,)
//     };
//     (parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             parent: $parent,
//             $crate::Level::WARN,
//             $name,
//             $($field)*
//         )
//     };
//     (parent: $parent:expr, $name:expr) => {
//         $crate::warn_span!(parent: $parent, $name,)
//     };
//     (target: $target:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             $crate::Level::WARN,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, $name:expr) => {
//         $crate::warn_span!(target: $target, $name,)
//     };
//     ($name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             $crate::Level::WARN,
//             $name,
//             $($field)*
//         )
//     };
//     ($name:expr) => {$crate::warn_span!($name,)};
// }
// /// Constructs a span at the error level.
// ///
// /// [Fields] and [attributes] are set using the same syntax as the [`span!`]
// /// macro.
// ///
// /// See [the top-level documentation][lib] for details on the syntax accepted by
// /// this macro.
// ///
// /// [lib]: crate#using-the-macros
// /// [attributes]: crate#configuring-attributes
// /// [Fields]: crate#recording-fields
// /// [`span!`]: span!
// ///
// /// # Examples
// ///
// /// ```rust
// /// # use tracing::{span, error_span, Level};
// /// # fn main() {
// /// error_span!("my_span");
// /// // is equivalent to:
// /// span!(Level::ERROR, "my_span");
// /// # }
// /// ```
// ///
// /// ```rust
// /// # use tracing::error_span;
// /// # fn main() {
// /// let span = error_span!("my span");
// /// span.in_scope(|| {
// ///     // do work inside the span...
// /// });
// /// # }
// /// ```
// #[macro_export]
// macro_rules! error_span {
//     (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             parent: $parent,
//             $crate::Level::ERROR,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, parent: $parent:expr, $name:expr) => {
//         $crate::error_span!(target: $target, parent: $parent, $name,)
//     };
//     (parent: $parent:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             parent: $parent,
//             $crate::Level::ERROR,
//             $name,
//             $($field)*
//         )
//     };
//     (parent: $parent:expr, $name:expr) => {
//         $crate::error_span!(parent: $parent, $name,)
//     };
//     (target: $target:expr, $name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: $target,
//             $crate::Level::ERROR,
//             $name,
//             $($field)*
//         )
//     };
//     (target: $target:expr, $name:expr) => {
//         $crate::error_span!(target: $target, $name,)
//     };
//     ($name:expr, $($field:tt)*) => {
//         $crate::span!(
//             target: module_path!(),
//             $crate::Level::ERROR,
//             $name,
//             $($field)*
//         )
//     };
//     ($name:expr) => {$crate::error_span!($name,)};
// }

/// Constructs a new `Event`.
///
/// The event macro is invoked with a `Level` and up to 32 key-value fields.
/// Optionally, a format string and arguments may follow the fields; this will
/// be used to construct an implicit field named "message".
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: crate#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::{event, Level};
///
/// # fn main() {
/// let data = (42, "forty-two");
/// let private_data = "private";
/// let error = "a bad error";
///
/// event!(Level::ERROR, %error, "Received error");
/// event!(
///     target: "app_events",
///     Level::WARN,
///     private_data,
///     ?data,
///     "App warning: {}",
///     error
/// );
/// event!(Level::INFO, the_answer = data.0);
/// # }
/// ```
///
// /// Note that *unlike `span!`*, `event!` requires a value for all fields. As
// /// events are recorded immediately when the macro is invoked, there is no
// /// opportunity for fields to be recorded later. A trailing comma on the final
// /// field is valid.
// ///
// /// For example, the following does not compile:
// /// ```rust,compile_fail
// /// # use tracing::{Level, event};
// /// # fn main() {
// /// event!(Level::INFO, foo = 5, bad_field, bar = "hello")
// /// #}
// /// ```
#[macro_export]
macro_rules! event {
    // (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> ({
    //     use $crate::__macro_support::Callsite as _;
    //     static CALLSITE: $crate::__macro_support::MacroCallsite = $crate::callsite2! {
    //         name: concat!(
    //             "event ",
    //             file!(),
    //             ":",
    //             line!()
    //         ),
    //         kind: $crate::metadata::Kind::EVENT,
    //         target: $target,
    //         level: $lvl,
    //         fields: $($fields)*
    //     };

    //     let enabled = $crate::level_enabled!($lvl) && {
    //         let interest = CALLSITE.interest();
    //         !interest.is_never() && CALLSITE.is_enabled(interest)
    //     };
    //     if enabled {
    //         (|value_set: $crate::field::ValueSet| {
    //             $crate::__tracing_log!(
    //                 $lvl,
    //                 CALLSITE,
    //                 &value_set
    //             );
    //             let meta = CALLSITE.metadata();
    //             // event with explicit parent
    //             $crate::Event::child_of(
    //                 $parent,
    //                 meta,
    //                 &value_set
    //             );
    //         })($crate::valueset!(CALLSITE.metadata().fields(), $($fields)*));
    //     } else {
    //         $crate::__tracing_log!(
    //             $lvl,
    //             CALLSITE,
    //             &$crate::valueset!(CALLSITE.metadata().fields(), $($fields)*)
    //         );
    //     }
    // });

    // (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
    //     $crate::event!(
    //         target: $target,
    //         parent: $parent,
    //         $lvl,
    //         { $($arg)+, $($fields)* }
    //     )
    // );
    // (target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
    //     $crate::event!(target: $target, parent: $parent, $lvl, { $($k).+ = $($fields)* })
    // );
    // (target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => (
    //     $crate::event!(target: $target, parent: $parent, $lvl, { $($arg)+ })
    // );
    (target: $target:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        if $crate::level_enabled!($lvl) {
            let metadata = $crate::metadata!($lvl);
            let (message, data) = $crate::valueset!(entry: $( $fields )*);
            let event = $crate::subscriber::Event::new(message, data, metadata);
            $crate::subscriber::dispatch(event);
        }
    });
    (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: $target,
            $lvl,
            { $($arg)+, $($fields)* }
        )
    );
    (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(target: $target, $lvl, { $($k).+ = $($fields)* })
    );
    (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $lvl, { $($arg)+ })
    );
    // (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $lvl,
    //         { $($arg)+, $($fields)* }
    //     )
    // );
    // (parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $lvl,
    //         { $($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $lvl:expr, ?$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $lvl,
    //         { ?$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $lvl:expr, %$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $lvl,
    //         { %$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $lvl:expr, $($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $lvl,
    //         { $($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $lvl:expr, %$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $lvl,
    //         { %$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $lvl,
    //         { ?$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $lvl:expr, $($arg:tt)+ ) => (
    //     $crate::event!(target: module_path!(), parent: $parent, $lvl, { $($arg)+ })
    // );
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($arg)+, $($fields)* }
        )
    );
    ($lvl:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+ = $($field)*}
        )
    );
    ($lvl:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { ?$($k).+, $($field)*}
        )
    );
    ($lvl:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { %$($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+) => (
        $crate::event!($lvl, ?$($k).+,)
    );
    ($lvl:expr, %$($k:ident).+) => (
        $crate::event!($lvl, %$($k).+,)
    );
    ($lvl:expr, $($k:ident).+) => (
        $crate::event!($lvl, $($k).+,)
    );
    ( $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: module_path!(), $lvl, { $($arg)+ })
    );
}

/// Constructs an event at the trace level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: event!
/// [lib]: crate#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::trace;
/// # #[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
/// # impl Position {
/// # const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
/// # fn dist(&self, other: Position) -> f32 {
/// #    let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
/// #    (x + y).sqrt()
/// # }
/// # }
/// # fn main() {
/// let pos = Position { x: 3.234, y: -1.223 };
/// let origin_dist = pos.dist(Position::ORIGIN);
///
/// trace!(position = ?pos, ?origin_dist);
/// trace!(
///     target: "app_events",
///     position = ?pos,
///     "x is {} and y is {}",
///     if pos.x >= 0.0 { "positive" } else { "negative" },
///     if pos.y >= 0.0 { "positive" } else { "negative" }
/// );
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    // (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    // );
    // (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, {}, $($arg)+)
    // );
    // (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         { $($field)+ },
    //         $($arg)+
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         { $($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         { ?$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         { %$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         { $($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         { ?$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         { %$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($arg:tt)+) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::TRACE,
    //         {},
    //         $($arg)+
    //     )
    // );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the debug level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: event!
/// [lib]: crate#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::debug;
/// # fn main() {
/// # #[derive(Debug)] struct Position { x: f32, y: f32 }
///
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!(?pos.x, ?pos.y);
/// debug!(target: "app_events", position = ?pos, "New position");
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    // (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    // );
    // (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, {}, $($arg)+)
    // );
    // (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         { $($field)+ },
    //         $($arg)+
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         { $($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         { ?$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         { %$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         { $($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         { ?$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         { %$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($arg:tt)+) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::DEBUG,
    //         {},
    //         $($arg)+
    //     )
    // );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the info level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: event!
/// [lib]: crate#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::info;
/// # // this is so the test will still work in no-std mode
/// # #[derive(Debug)]
/// # pub struct Ipv4Addr;
/// # impl Ipv4Addr { fn new(o1: u8, o2: u8, o3: u8, o4: u8) -> Self { Self } }
/// # fn main() {
/// # struct Connection { port: u32, speed: f32 }
/// use tracing::field;
///
/// let addr = Ipv4Addr::new(127, 0, 0, 1);
/// let conn = Connection { port: 40, speed: 3.20 };
///
/// info!(conn.port, "connected to {:?}", addr);
/// info!(
///     target: "connection_events",
///     ip = ?addr,
///     conn.port,
///     ?conn.speed,
/// );
/// # }
/// ```
#[macro_export]
macro_rules! info {
    // (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($field)* }, $($arg)*)
    // );
    // (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, {}, $($arg)+)
    // );
    // (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         { $($field)+ },
    //         $($arg)+
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         { $($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         { ?$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         { %$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         { $($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         { ?$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         { %$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($arg:tt)+) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::INFO,
    //         {},
    //         $($arg)+
    //     )
    // );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the warn level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: event!
/// [lib]: crate#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::warn;
/// # fn main() {
///
/// let warn_description = "Invalid Input";
/// let input = &[0x27, 0x45];
///
/// warn!(?input, warning = warn_description);
/// warn!(
///     target: "input_events",
///     warning = warn_description,
///     "Received warning for input: {:?}", input,
/// );
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    // (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($field)* }, $($arg)*)
    // );
    // (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, {}, $($arg)+)
    // );
    // (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         { $($field)+ },
    //         $($arg)+
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         { $($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         { ?$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         { %$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         { $($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         { ?$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         { %$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($arg:tt)+) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::WARN,
    //         {},
    //         $($arg)+
    //     )
    // );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the error level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: event!
/// [lib]: crate#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::error;
/// # fn main() {
///
/// let (err_info, port) = ("No connection", 22);
///
/// error!(port, error = %err_info);
/// error!(target: "app_events", "App Error: {}", err_info);
/// error!({ info = err_info }, "error on port: {}", port);
/// # }
/// ```
#[macro_export]
macro_rules! error {
    // (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    // );
    // (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    // );
    // (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    //     $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, {}, $($arg)+)
    // );
    // (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         { $($field)+ },
    //         $($arg)+
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         { $($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         { ?$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         { %$($k).+ = $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         { $($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         { ?$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         { %$($k).+, $($field)*}
    //     )
    // );
    // (parent: $parent:expr, $($arg:tt)+) => (
    //     $crate::event!(
    //         target: module_path!(),
    //         parent: $parent,
    //         $crate::Level::ERROR,
    //         {},
    //         $($arg)+
    //     )
    // );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)* })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { ?$($k).+ $($field)* })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { %$($k).+ $($field)* })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            {},
            $($arg)+
        )
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! valueset {

    // === base case ===
    (@ { $(,)* $($val:expr),* $(,)* }, $message:ident, $next:expr $(,)*) => {
        [ $($val),* ].into_iter().collect()
    };

    // === recursive case (more tts) ===

    // TODO(#1138): determine a new syntax for uninitialized span fields, and
    // re-enable this.
    // (@{ $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = _, $($rest:tt)*) => {
    //     $crate::valueset!($message:ident, @ { $($out),*, (&$next, None) }, $next, $($rest)*)
    // };
    // foo = ?bar ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{:?}", $val).into()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // foo = %bar ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{}", $val).into()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // foo = bar ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), serde_json::to_value(&$val).unwrap()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // foo ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), serde_json::to_value(&$($k).+).unwrap()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // ?foo ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, ?$($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{:?}", $($k).+).into()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // %foo ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, %$($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{}", $($k).+).into()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // foo = ?bar
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+ = ?$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{:?}", $val).into()) },
            $message,
            $next,
        )
    };
    // foo = %bar
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+ = %$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{}", $val).into()) },
            $message,
            $next,
        )
    };
    // foo = bar
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+ = $val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), serde_json::to_value(&$val).unwrap()) },
            $message,
            $next,
        )
    };
    // foo
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), serde_json::to_value(&$($k).+).unwrap()) },
            $message,
            $next,
        )
    };
    // ?foo
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, ?$($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{:?}", $($k).+).into()) },
            $message,
            $next,
        )
    };
    // %foo
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, %$($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (stringify!($($k).+).to_string(), format!("{}", $($k).+).into()) },
            $message,
            $next,
        )
    };

    // Handle literal names
    // "foo" = ?bar ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $k:literal = ?$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, ($k.to_string(), format!("{:?}", $val).into()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // "foo" = %bar ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $k:literal = %$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, ($k.to_string(), format!("{}", $val).into()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // "foo" = bar ...
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $k:literal = $val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, ($k.to_string(), serde_json::to_value(&$val).unwrap()) },
            $message,
            $next,
            $($rest)*
        )
    };
    // "foo" = ?bar
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $k:literal = ?$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, ($k.to_string(), format!("{:?}", $val).into()) },
            $message,
            $next,
        )
    };
    // "foo" = %bar
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $k:literal = %$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, ($k.to_string(), format!("{}", $val).into()) },
            $message,
            $next,
        )
    };
    // "foo" = bar
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $k:literal = $val:expr) => {
        $crate::valueset!(
            @ { $($out),*, ($k.to_string(), serde_json::to_value(&$val).unwrap()) },
            $message,
            $next,
        )
    };

    // Remainder is unparseable, but exists --- must be format args!
    (@ { $(,)* $($out:expr),* }, $message:ident, $next:expr, $($rest:tt)+) => {{
        $message = format!($($rest)+);
        $crate::valueset!(
            @ { $($out),* },
            $message,
            $next,
        )
    }};

    // === entry ===
    (entry: $($kvs:tt)+) => {
        {
            extern crate serde_json;
            extern crate std;

            use std::convert::Into;
            use std::format;
            use std::iter::{Iterator, IntoIterator};
            use std::string::{ToString, String};

            let mut message = String::new();
            let data: serde_json::Map<String, serde_json::Value> = $crate::valueset!(
                @ { },
                message,
                (),
                $($kvs)+
            );
            (message, data)
        }
    };
    // () => {
    //     {
    //         $fields.value_set(&[])
    //     }
    // };
}

#[macro_export]
#[doc(hidden)]
macro_rules! level_enabled {
    ($lvl:expr) => {
        $lvl <= $crate::level::STATIC_MAX_LEVEL && $lvl <= $crate::level::LevelFilter::current()
    };
}
