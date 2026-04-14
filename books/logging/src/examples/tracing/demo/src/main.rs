use tracing::{Level, debug, error, info, trace, warn};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Set up a tracing subscriber that logs to STDOUT.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        // Change it to write to STDERR
        //.with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Emit some log messages.
    trace!("trace level");
    debug!("debug level");

    let answer = 42;
    info!(answer, "info level");
    warn!("warn level");
    error!("error level");
}
