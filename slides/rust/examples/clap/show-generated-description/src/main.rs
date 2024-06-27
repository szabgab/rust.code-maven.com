use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(about = get_about())]
struct Cli {
}

fn main() {
    let _args = Cli::parse();
    println!("Use the --help flag");
}

fn get_about() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    format!("The description generated at {}", since_the_epoch.as_nanos())
}
