use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(about = get_about())]
struct Cli {
    #[arg(long)]
    host: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.host);
}

fn get_about() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("{}", since_the_epoch.as_nanos())
}