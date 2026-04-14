use tracing::{Level, debug, error, info, trace, warn};
use tracing_subscriber::FmtSubscriber;

use which::which;

fn main() {
    // prints to STDOUT
    //let subscriber = FmtSubscriber::builder()
    //    .with_max_level(Level::TRACE)
    //    .finish();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // demo:
    // which::finder:
    // which::checker


   //tracing_subscriber::registry()
   //.with(
   //    tracing_subscriber::EnvFilter::try_from_default_env()
   //        //.unwrap_or_else(|_| "axum_logging=warn,tower_http=debug".into()),
   //        .unwrap_or_else(|_| "axum_logging=warn,tower_http=debug,axum::serve=trace".into()),
   //)
   //.with(tracing_subscriber::fmt::layer())
   //.init();


    trace!("trace level");
    debug!("debug level");

    let answer = 42;
    info!(answer, "info level");
    warn!("warn level");
    error!("error level");


    let _result = which("rustc").unwrap();
}
