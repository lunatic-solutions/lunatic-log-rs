use lunatic::{serializer, spawn};
use lunatic_log::{info, level_enabled, level_filters::STATIC_MAX_LEVEL, Dispatch};
use lunatic_tracing_core::{dispatch::DispatchMessage, LevelFilter};

fn main() {
    // Initialize subscriber
    // lunatic_log::init(FmtSubscriber::new(LevelFilter::Info));

    LevelFilter::set_max(LevelFilter::TRACE);

    let collector = spawn!(|mailbox: Mailbox<DispatchMessage, serializer::Json>| {
        loop {
            let msg = mailbox.receive();
            dbg!(msg);
            // match msg {
            //     RegisterCallsite(MessageRequest<Metadata, collect::Interest>),
            //     OnRegisterDispatch(Dispatch),
            //     MaxLevelHint(MessageRequest<(), Option<LevelFilter>>),
            //     NewSpan(MessageRequest<span::Attributes, span::Id>),
            //     CurrentSpan(MessageRequest<(), span::Current>),
            //     Enabled(MessageRequest<Metadata, bool>),
            //     Event(Event),
            //     Enter(span::Id),
            //     Exit(span::Id),
            //     CloneSpan(span::Id),
            //     TryClose(MessageRequest<span::Id, bool>),
            //     Record {
            //         span: span::Id,
            //         values: span::Record,
            //     },
            //     RecordFollowsFrom {
            //         span: span::Id,
            //         follows: span::Id,
            //     },
            // }
        }
    });
    let dispatcher = Dispatch::some(collector);

    lunatic_log::dispatch::set_global_default(&dispatcher).unwrap();

    // Log message
    info!("Hello World");

    // {
    //     use lunatic_log::Callsite as _;
    //     let mut callsite: lunatic_log::Callsite = {
    //         let metadata: lunatic_log::Metadata = {
    //             lunatic_log::metadata::Metadata::new(
    //                 ("event :0"),
    //                 ("module::path"),
    //                 (lunatic_log::Level::INFO),
    //                 Some(""),
    //                 Some(0),
    //                 Some("module::path"),
    //                 lunatic_log::field::FieldSet::new(
    //                     (&[("message")]),
    //                     lunatic_log::callsite::Identifier(0),
    //                 ),
    //                 (lunatic_log::metadata::Kind::EVENT),
    //             )
    //         };
    //         lunatic_log::Callsite::new(metadata)
    //     };
    //     let enabled = (lunatic_log::Level::INFO) <= lunatic_log::level_filters::STATIC_MAX_LEVEL
    //         && (lunatic_log::Level::INFO) <= lunatic_log::level_filters::LevelFilter::current()
    //         && {
    //             let interest = callsite.interest();
    //             !interest.is_never() && callsite.is_enabled(interest)
    //         };
    //     if enabled {
    //         (|value_set: lunatic_log::field::ValueSet| {
    //             let meta = callsite.metadata().clone();
    //             lunatic_log::Event::dispatch(meta, value_set);
    //         })({
    //             #[allow(unused_imports)]
    //             use lunatic_log::field::{debug, display, Value};
    //             let mut iter = (callsite.metadata().fields()).iter();
    //             (callsite.metadata().fields()).value_set(
    //                 (<[_]>::into_vec(
    //                     #[rustc_box]
    //                     lunatic_log::boxed::Box::new([(
    //                         (iter.next().expect("FieldSet corrupted (this is a bug)")),
    //                         Some(
    //                             ({
    //                                 let res = lunatic_log::fmt::format(
    //                                     lunatic_log::fmt::Arguments::new_v1(&[], &[]),
    //                                 );
    //                                 res
    //                             }
    //                             .into()),
    //                         ),
    //                     )]),
    //                 )),
    //             )
    //         });
    //     } else {
    //     }
    // }

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
