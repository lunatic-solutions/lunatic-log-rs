use lunatic_log::{debug, error, info, subscriber::fmt::FmtSubscriber, trace, warn, LevelFilter};

fn main() {
    // Initialize subscriber
    lunatic_log::init(FmtSubscriber::new(LevelFilter::Trace).pretty());

    // Log message
    error!("Error");
    warn!("Warn");
    info!("Info");
    debug!("Debug");
    trace!("Trace");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
