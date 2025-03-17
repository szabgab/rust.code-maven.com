use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Set up a tracing subscriber that logs to stdout.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");


    // Emit some log messages.
    info!("hello world");

    let number_of_yaks = 3;
    debug!(number_of_yaks, "preparing to shave yaks");
}
