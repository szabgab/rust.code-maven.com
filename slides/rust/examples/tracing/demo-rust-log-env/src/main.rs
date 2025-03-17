use tracing::{debug, error, info, trace, warn};

fn main() {
    // Set up a tracing subscriber that logs to stdout.
    // User RUST_LOG to control the logging level.
    // RUST_LOG=trace
    // RUST_LOG=error
    tracing_subscriber::fmt::init();

    // Emit some log messages.
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}
