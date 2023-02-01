/// Logs a message with a specified level.
///
/// You should use [`error`], [`warn`], [`info`], [`debug`], [`trace`] macros instead.
#[macro_export]
macro_rules! log {
    // log!(target: "my_target", Level::Info; "a {} event", "log");
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        if let Some(proc) = $crate::__lookup_logging_process() {
            let metadata = $crate::Metadata::new(
                concat!(
                    "event ",
                    file!(),
                    ":",
                    line!()
                ).to_string(),
                $target.into(),
                $lvl,
                Some(module_path!().to_string()),
                Some(file!().to_string()),
                Some(line!()),
            );
            let message = format!($($arg)+);
            let event = $crate::Event::new(message, metadata);
            proc.send(event)
        }
    });

    // log!(Level::Info, "a log event")
    ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: module_path!(), $lvl, $($arg)+));
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```
/// use lunatic_log::error;
///
/// # fn main() {
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {} on port {}", err_info, port);
/// error!(target: "app_events", "App Error: {}, Port: {}", err_info, 22);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! error {
    // error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // error!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => (log!(target: $target, $crate::Level::Error, $($arg)+));

    // error!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::Level::Error, $($arg)+))
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```
/// use lunatic_log::warn;
///
/// # fn main() {
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {}!", warn_description);
/// warn!(target: "input_events", "App received warning: {}", warn_description);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! warn {
    // warn!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // warn!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => (log!(target: $target, $crate::Level::Warn, $($arg)+));

    // warn!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::Level::Warn, $($arg)+))
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```
/// use lunatic_log::info;
///
/// # fn main() {
/// # struct Connection { port: u32, speed: f32 }
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
/// info!(target: "connection_events", "Successfull connection, port: {}, speed: {}",
///       conn_info.port, conn_info.speed);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! info {
    // info!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // info!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => (log!(target: $target, $crate::Level::Info, $($arg)+));

    // info!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::Level::Info, $($arg)+))
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```
/// use lunatic_log::debug;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! debug {
    // debug!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // debug!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => (log!(target: $target, $crate::Level::Debug, $($arg)+));

    // debug!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::Level::Debug, $($arg)+))
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```
/// use lunatic_log::trace;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// trace!(target: "app_events", "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// # }
/// ```
#[macro_export(local_inner_macros)]
macro_rules! trace {
    // trace!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // trace!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => (log!(target: $target, $crate::Level::Trace, $($arg)+));

    // trace!("a {} event", "log")
    ($($arg:tt)+) => (log!($crate::Level::Trace, $($arg)+))
}
