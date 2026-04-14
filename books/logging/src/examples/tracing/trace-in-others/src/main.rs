use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use which::which;

fn main() {
    tracing_subscriber::registry()
        //.with(
        //    tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        //        format!(
        //            "{}=trace,which::checker=debug,which::finder=trace",
        //            env!("CARGO_CRATE_NAME")
        //        )
        //        .into()
        //    }),
        //)
        .with(tracing_subscriber::fmt::layer())
        .init();

    trace!("trace level");
    debug!("debug level");
    info!("info level");
    warn!("warn level");
    error!("error level");

    let _result = which("rustc").unwrap();
}
