mod common;

use common::*;
use lunatic::test;
use lunatic_tracing_core::dispatch::*;

#[test]
fn global_dispatch() {
    let dispatcher_a = Dispatch::spawn(TestCollectorA);
    set_global_default(&dispatcher_a).expect("global dispatch set failed");
    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_a.collector().map(|process| process.id())
        )
    });

    let dispatcher_b = Dispatch::spawn(TestCollectorB);
    with_default(&dispatcher_b, || {
        get_default(|current| {
            assert_eq!(
                current.collector().map(|process| process.id()),
                dispatcher_b.collector().map(|process| process.id())
            )
        });
    });

    get_default(|current| {
        assert_eq!(
            current.collector().map(|process| process.id()),
            dispatcher_a.collector().map(|process| process.id())
        )
    });

    set_global_default(&dispatcher_a).expect_err("double global dispatch set succeeded");
}
