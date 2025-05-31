use clap::builder::{ArgPredicate, OsStr};
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long)]
    log_to_file: bool,

    #[arg(
        long,
        default_value_if("log_to_file", ArgPredicate::Equals(OsStr::from("true")), "my.log")
    )]
    log_file: Option<String>,
}

fn main() {
    let args = Cli::parse();

    println!("Args: {args:?}");
}
