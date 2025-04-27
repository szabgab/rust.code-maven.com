use std::io::Write;
use std::env;

use env_logger::Builder;
use chrono::Local;

fn main() {
    init_logger();

    log::trace!("This is a sample trace.");
    log::debug!("This is a sample debug.");
    log::info!("This is a sample info.");
    log::warn!("This is a sample warn.");
    log::error!("This is a sample error.");
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });
    let log_level = env::var("RUST_LOG").unwrap_or(String::from("info"));
    builder.parse_filters(&log_level);

    builder.init();
}

