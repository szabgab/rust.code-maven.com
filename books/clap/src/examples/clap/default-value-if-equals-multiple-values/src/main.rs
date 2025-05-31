use clap::builder::{ArgPredicate, OsStr};
use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Devel,
    Test,
    Release,
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long)]
    mode: Mode,

    #[arg(
        long,
        default_value_if("mode", ArgPredicate::Equals(OsStr::from("devel")), "1"),
        default_value_if("mode", ArgPredicate::Equals(OsStr::from("test")), "2"),
        default_value_t = 0
    )]
    log_level: u8,
}

fn main() {
    let args = Cli::parse();

    println!("Args: {args:?}");
}
