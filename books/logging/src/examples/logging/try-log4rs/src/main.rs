fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    log::trace!("This is a sample trace.");
    log::debug!("This is a sample debug.");
    log::info!("This is a sample info.");
    log::warn!("This is a sample warn.");
    log::error!("This is a sample error.");

    tools::do_something();
}

mod tools {
    pub fn do_something() {
        log::trace!("do_something trace.");
        log::debug!("do_something debug.");
        log::info!("do_something info.");
        log::warn!("do_something warn.");
        log::error!("do_something error.");
    }
}
