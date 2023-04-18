//! Subscriber that prints to stdout/stderr.
//!
//! Supports pretty printing with colors.

use std::fmt::Write;

use ansi_term::{Color, Style};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

use crate::{
    level::{Level, LevelFilter},
    metadata::Metadata,
};

use super::{init_subscriber, Event, Subscriber, SubscriberAlreadyExistsError, SubscriberProcess};

const GRAY: Color = Color::RGB(135, 135, 135);

/// A subscriber printing to stdout/stderr.
///
/// # Basic example
///
/// ```
/// lunatic_log::init(FmtSubscriber::new(LevelFilter::Info));
/// ```
///
/// # Pretty example
///
/// ```
/// lunatic_log::init(FmtSubscriber::new(LevelFilter::Info).pretty());
/// ```
#[derive(Serialize, Deserialize)]
pub struct FmtSubscriber {
    color: bool,
    file: bool,
    level: bool,
    level_filter: LevelFilter,
    line_number: bool,
    target: bool,
    time: bool,
    time_format: Option<String>,
}

impl Default for FmtSubscriber {
    fn default() -> Self {
        Self {
            color: false,
            file: false,
            level: false,
            level_filter: LevelFilter::OFF,
            line_number: false,
            target: false,
            time: false,
            time_format: None,
        }
    }
}

impl FmtSubscriber {
    /// Creates an instance of [`FmtSubscriber`].
    pub fn new(level_filter: LevelFilter) -> Self {
        FmtSubscriber {
            level_filter,
            ..Default::default()
        }
    }

    /// Configures logging to be pretty with colors, filenames, and more.
    pub fn pretty(mut self) -> Self {
        self.color = true;
        self.file = true;
        self.level = true;
        self.line_number = true;
        self.target = true;
        self.time = true;
        self
    }

    /// Enables printing color.
    pub fn with_color(mut self, color: bool) -> Self {
        self.color = color;
        self
    }

    /// Print filename where log originated.
    pub fn with_file(mut self, file: bool) -> Self {
        self.file = file;
        self
    }

    /// Print the log level.
    pub fn with_level(mut self, level: bool) -> Self {
        self.level = level;
        self
    }

    /// Print the line number where log originated.
    pub fn with_line_number(mut self, line_number: bool) -> Self {
        self.line_number = line_number;
        self
    }

    /// Print the target of the log.
    pub fn with_target(mut self, target: bool) -> Self {
        self.target = target;
        self
    }

    /// Print the time with the log.
    pub fn with_time(mut self, time: bool) -> Self {
        self.time = time;
        self
    }

    /// Customize the time format.
    ///
    /// This must be in the [`strftime`](https://docs.rs/chrono/latest/chrono/format/strftime/index.html) format supported by [chrono](https://docs.rs/chrono).
    pub fn with_time_format(mut self, time_format: impl Into<String>) -> Self {
        self.time_format = Some(time_format.into());
        self
    }

    /// Initializes as the global subscriber.
    ///
    /// Note, this will cause a panic if a global subscriber has already been initialized.
    /// Use the [`try_init`] to handle this error.
    pub fn init(self) {
        self.try_init().unwrap();
    }

    /// Initializes as the global subscriber, returning an error if a global subscriber has already been initialized.
    pub fn try_init(self) -> Result<SubscriberProcess, SubscriberAlreadyExistsError> {
        init_subscriber(self)
    }

    fn write_data(&self, line: &mut String, data: &Map<String, Value>) {
        for (i, (k, v)) in data.iter().enumerate() {
            if i > 0 {
                write!(line, " ").unwrap();
            }
            write!(line, "{}=", Style::new().italic().paint(k)).unwrap();
            self.write_value(line, v);
        }
    }

    fn write_value(&self, line: &mut String, value: &Value) {
        match value {
            Value::Null => self.write_null(line),
            Value::Bool(b) => self.write_bool(line, *b),
            Value::Number(n) => self.write_number(line, n),
            Value::String(s) => self.write_string(line, s),
            Value::Array(a) => self.write_array(line, a),
            Value::Object(o) => self.write_object(line, o),
        }
    }

    fn write_null(&self, line: &mut String) {
        if self.color {
            write!(line, "{}", GRAY.paint("null")).unwrap();
        } else {
            write!(line, "null").unwrap();
        }
    }

    fn write_bool(&self, line: &mut String, b: bool) {
        if self.color {
            if b {
                write!(line, "{}", Color::Green.paint("true")).unwrap();
            } else {
                write!(line, "{}", Color::Red.paint("false")).unwrap();
            }
        } else {
            write!(line, "{b}").unwrap();
        }
    }

    fn write_number(&self, line: &mut String, n: &Number) {
        write!(line, "{n}").unwrap();
    }

    fn write_string(&self, line: &mut String, s: &str) {
        write!(line, r#""{s}""#).unwrap();
    }

    fn write_array(&self, line: &mut String, values: &[Value]) {
        write!(line, "[ ").unwrap();
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                write!(line, ", ").unwrap();
            }
            self.write_value(line, value);
        }
        write!(line, " ]").unwrap();
    }

    fn write_object(&self, line: &mut String, object: &Map<String, Value>) {
        write!(line, "{{ ").unwrap();
        for (i, (k, v)) in object.iter().enumerate() {
            if i > 0 {
                write!(line, ", ").unwrap();
            }
            write!(line, "{k}: ").unwrap();
            self.write_value(line, v);
        }
        write!(line, " }}").unwrap();
    }
}

impl Subscriber for FmtSubscriber {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn event(&self, event: &Event) {
        let mut line = String::new();
        macro_rules! insert_space {
            () => {
                if !line.is_empty() {
                    line.push(' ');
                }
            };
        }

        if self.time {
            insert_space!();
            let now = Utc::now();
            let now_string = now
                .format(
                    self.time_format
                        .as_deref()
                        .unwrap_or("%Y-%m-%dT%H:%M:%S%.6fZ"),
                )
                .to_string();
            if self.color {
                write!(line, "{}", GRAY.paint(now_string)).unwrap();
            } else {
                write!(line, "{now_string}").unwrap();
            }
        }

        if self.level {
            insert_space!();
            if self.color {
                let level_string = match event.metadata().level() {
                    Level::ERROR => Color::Red,
                    Level::WARN => Color::Yellow,
                    Level::INFO => Color::Green,
                    Level::DEBUG => Color::Blue,
                    Level::TRACE => Color::Purple,
                }
                .paint(event.metadata().level().as_str());
                for _ in 0..(5 - event.metadata().level().as_str().len()) {
                    insert_space!();
                }
                write!(line, "{level_string}").unwrap();
            } else {
                let level_string = event.metadata().level().as_str();
                for _ in 0..(5 - event.metadata().level().as_str().len()) {
                    insert_space!();
                }
                write!(line, "{level_string}").unwrap();
            };
        }

        if self.target {
            insert_space!();
            if self.color {
                write!(
                    line,
                    "{}",
                    GRAY.paint(format!("{}:", event.metadata().module_path()))
                )
                .unwrap();
            } else {
                write!(line, "{}:", event.metadata().module_path()).unwrap();
            }
        }

        if self.file {
            insert_space!();
            if self.color {
                write!(
                    line,
                    "{}",
                    GRAY.paint(format!("{}:", event.metadata().file()))
                )
                .unwrap();
            } else {
                write!(line, "{}:", self.file).unwrap();
            }
        }

        if self.line_number {
            if self.color {
                write!(
                    line,
                    "{}",
                    GRAY.paint(format!("{}:", event.metadata().line()))
                )
                .unwrap();
            } else {
                write!(line, "{}:", event.metadata().line()).unwrap();
            }
        }

        if !event.data().is_empty() {
            insert_space!();
            self.write_data(&mut line, event.data());
        }

        if !event.message().is_empty() {
            insert_space!()
        }

        if event.metadata().level() == Level::ERROR {
            eprintln!("{line}{}", event.message());
        } else {
            println!("{line}{}", event.message());
        }
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(self.level_filter)
    }
}
