use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(long, help = "Set logging level (debug, info, warn, error). Default is 'warn'")]
    log: Option<log::Level>,
}


fn main() {
    let args = Cli::parse();

    let mut builder = env_logger::Builder::new();

    let log_level = if let Some(level) = args.log {
        level.to_string()
    } else {
        std::env::var("RUST_LOG").unwrap_or(String::from("warn"))
    };

    builder.parse_filters(&log_level);
    builder.init();


    log::debug!("Debug level log");
    log::info!("Info level log");
    log::warn!("Warn level log");
    log::error!("Error level log");
}
