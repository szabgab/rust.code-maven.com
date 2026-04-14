use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{
    Layer, filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt,
};

fn main() {
    let log_filename = "tracing.log";
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(
                    std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(log_filename)
                        .unwrap(),
                )
                .with_filter(LevelFilter::DEBUG),
        )
        // Enable this to also log to STDOUT:
        //.with(tracing_subscriber::fmt::layer())
        .init();

    trace!("trace level");
    debug!("debug level");

    let answer = 42;
    info!(answer, "info level");
    warn!("warn level");
    error!("error level");
}
