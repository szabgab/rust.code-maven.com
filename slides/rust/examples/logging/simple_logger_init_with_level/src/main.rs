fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    log::trace!("This is a sample trace.");
    log::debug!("This is a sample debug.");
    log::info!("This is a sample info.");
    log::warn!("This is a sample warn.");
    log::error!("This is a sample error.");
}

