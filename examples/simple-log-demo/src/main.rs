use log::error;
use log::{debug, info};

fn main() {
    // Default initialization, all the log levels.
    // simple_logger::SimpleLogger::new().init().unwrap();

    // Set the log level inside the code.
    // simple_logger::init_with_level(log::Level::Warn).unwrap();

    // Allow the user to set the log level as an environment variable:
    // RUST_LOG=warn
    // simple_logger::init_with_env().unwrap();

    // Allow the user to provide the log level on the command line:
    // crate run warn
    simple_logger::init_with_level(get_log_level()).unwrap();

    println!("Hello, world!");

    log::trace!("This is a sample trace.");
    log::debug!("This is a sample debug.");
    log::info!("This is a sample info.");
    log::warn!("This is a sample warn.");
    log::error!("This is a sample error.");

    debug!("Another debug.");
    info!("Another info.");
    error!("Another error.");
}

use std::str::FromStr;

fn get_log_level() -> log::Level {
    log::Level::from_str(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .unwrap_or(&String::from("trace"))
            .to_lowercase()
            .as_str(),
    )
    .unwrap_or(log::Level::Trace)
}
