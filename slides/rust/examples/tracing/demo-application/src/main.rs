use demo_lib::add;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Set up a tracing subscriber that logs to stdout.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Before");

    add(2, 3);

    info!("after");
}
