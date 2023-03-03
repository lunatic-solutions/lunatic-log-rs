mod common;

use common::*;
use lunatic::test;
use lunatic_tracing_core::dispatch::*;

#[test]
fn set_default_dispatch() {
    let dispatcher_a = Dispatch::spawn(TestCollectorA);
    set_global_default(&dispatcher_a).expect("global dispatch set failed");
    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_a.collector().map(|process| process.id())
        )
    });

    let dispatcher_b = Dispatch::spawn(TestCollectorB);
    let guard = set_default(&dispatcher_b);
    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_b.collector().map(|process| process.id())
        )
    });

    // Drop the guard, setting the dispatch back to the global dispatch
    drop(guard);

    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_a.collector().map(|process| process.id())
        )
    });
}

#[test]
fn nested_set_default() {
    let dispatcher_a = Dispatch::spawn(TestCollectorA);
    let _guard = set_default(&dispatcher_a);
    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_a.collector().map(|process| process.id())
        )
    });

    let dispatcher_b = Dispatch::spawn(TestCollectorB);
    let inner_guard = set_default(&dispatcher_b);
    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_b.collector().map(|process| process.id())
        )
    });

    drop(inner_guard);
    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_a.collector().map(|process| process.id())
        )
    });
}
