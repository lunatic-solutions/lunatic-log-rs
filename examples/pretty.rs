use lunatic_log::{debug, error, info, subscriber::fmt::FmtSubscriber, trace, warn, LevelFilter};

fn main() {
    // Initialize subscriber
    FmtSubscriber::new(LevelFilter::TRACE).pretty().init();

    // Log message
    error!("Error");
    warn!("Warn");
    info!("Info");
    debug!("Debug");
    trace!("Trace");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
