use log::error;
use log::{debug, info};

fn main() {
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
