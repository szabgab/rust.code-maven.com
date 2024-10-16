use clap::builder::ArgPredicate;
use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, default_value_if("log_file", ArgPredicate::IsPresent, "true"))]
    log_to_file: bool,

    #[arg(long, default_value = "my.log")]
    log_file: String,
}

fn main() {
    let args = Cli::parse();

    println!("Args: {args:?}");
}
