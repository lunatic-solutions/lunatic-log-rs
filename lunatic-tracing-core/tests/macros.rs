use lunatic::test;
use lunatic_tracing_core::{
    metadata,
    metadata::{Kind, Level},
};

#[test]
fn metadata_macro_api() {
    // This test should catch any inadvertent breaking changes
    // caused by changes to the macro.

    let _metadata = metadata! {
        name: "test_metadata",
        target: "test_target",
        level: Level::DEBUG,
        fields: &["foo", "bar", "baz"],
        callsite: 0,
        kind: Kind::SPAN,
    };
    let _metadata = metadata! {
        name: "test_metadata",
        target: "test_target",
        level: Level::TRACE,
        fields: &[],
        callsite: 0,
        kind: Kind::EVENT,
    };
    let _metadata = metadata! {
        name: "test_metadata",
        target: "test_target",
        level: Level::INFO,
        fields: &[],
        callsite: 0,
        kind: Kind::EVENT
    };
}
