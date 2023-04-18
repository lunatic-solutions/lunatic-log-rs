//! TODO

use opentelemetry::{
    global,
    trace::{Span, Tracer},
    Context, KeyValue, StringValue,
};
use serde_json::Map;

use crate::{level::LevelFilter, metadata::Metadata};

use super::{Event, Subscriber};

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
pub struct OpenTelemetrySubscriber<T>
where
    T: Tracer,
{
    tracer: T,
    level_filter: LevelFilter,
}

impl<T> OpenTelemetrySubscriber<T>
where
    T: Tracer,
{
    /// Creates an instance of [`OpenTelemetrySubscriber`].
    pub fn new(tracer: T, level_filter: LevelFilter) -> Self {
        OpenTelemetrySubscriber {
            tracer,
            level_filter,
        }
    }
}

impl<T> Subscriber for OpenTelemetrySubscriber<T>
where
    T: Tracer,
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn event(&self, event: &Event) {
        let name = format!(
            "event {}:{}",
            event.metadata().file(),
            event.metadata().line()
        );
        let mut span = self.tracer.start(name.clone());
        span.add_event(name, data_to_opentelemetry(event.data()));
        println!("ending");
        span.end();
        println!("shutdown");
        global::shutdown_tracer_provider();
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(self.level_filter)
    }
}

// fn init_meter() -> PrometheusExporter {
//     let controller = controllers::basic(
//         processors::factory(
//             selectors::simple::histogram([1.0, 2.0, 5.0, 10.0, 20.0, 50.0]),
//             aggregation::cumulative_temporality_selector(),
//         )
//         .with_memory(true),
//     )
//     .build();

//     opentelemetry_prometheus::exporter(controller).init()
// }

fn data_to_opentelemetry(data: &Map<String, serde_json::Value>) -> Vec<KeyValue> {
    data.into_iter()
        .map(|(k, v)| KeyValue {
            key: k.to_string().into(),
            value: json_to_opentelemetry(v.clone()), // TODO: Remove this clone
        })
        .collect()
}

fn json_to_opentelemetry(value: serde_json::Value) -> opentelemetry::Value {
    match value {
        serde_json::Value::Null => "null".into(),
        serde_json::Value::Bool(b) => opentelemetry::Value::Bool(b),
        serde_json::Value::Number(n) => n
            .as_f64()
            .map(opentelemetry::Value::F64)
            .or_else(|| n.as_i64().map(opentelemetry::Value::I64))
            .unwrap(),
        serde_json::Value::String(s) => s.into(),
        serde_json::Value::Array(a) => {
            let first_type = a.first();
            let valid_ot_array = a.iter().skip(1).all(|value| match first_type {
                Some(serde_json::Value::Null) => false,
                Some(serde_json::Value::Bool(_)) => value.is_boolean(),
                Some(serde_json::Value::Number(n)) if n.is_f64() => value.is_f64(),
                Some(serde_json::Value::Number(n)) if n.is_i64() => value.is_i64(),
                Some(serde_json::Value::String(_)) => value.is_string(),
                Some(serde_json::Value::Array(_)) => false,
                Some(serde_json::Value::Object(_)) => false,
                _ => false,
            });
            if valid_ot_array {
                match first_type.unwrap() {
                    serde_json::Value::Bool(_) => opentelemetry::Value::Array(
                        a.into_iter()
                            .map(|value| match value {
                                serde_json::Value::Bool(_) => value.as_bool(),
                                _ => None,
                            })
                            .collect::<Option<Vec<_>>>()
                            .unwrap()
                            .into(),
                    ),
                    serde_json::Value::Number(n) if n.is_f64() => opentelemetry::Value::Array(
                        a.into_iter()
                            .map(|value| match value {
                                serde_json::Value::Number(_) => value.as_f64(),
                                _ => None,
                            })
                            .collect::<Option<Vec<_>>>()
                            .unwrap()
                            .into(),
                    ),
                    serde_json::Value::Number(n) if n.is_i64() => opentelemetry::Value::Array(
                        a.into_iter()
                            .map(|value| match value {
                                serde_json::Value::Number(_) => value.as_i64(),
                                _ => None,
                            })
                            .collect::<Option<Vec<_>>>()
                            .unwrap()
                            .into(),
                    ),
                    serde_json::Value::String(_) => opentelemetry::Value::Array(
                        a.into_iter()
                            .map(|value| match value {
                                serde_json::Value::String(_) => match value {
                                    serde_json::Value::String(s) => Some(StringValue::from(s)),
                                    _ => None,
                                },
                                _ => None,
                            })
                            .collect::<Option<Vec<_>>>()
                            .unwrap()
                            .into(),
                    ),
                    _ => unreachable!("we already checked for other types"),
                }
            } else {
                opentelemetry::Value::Array(
                    a.into_iter()
                        .map(|value| StringValue::from(value.to_string()))
                        .collect::<Vec<_>>()
                        .into(),
                )
            }
        }
        serde_json::Value::Object(o) => serde_json::to_string(&o).unwrap().into(),
    }
}
