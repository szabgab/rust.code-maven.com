use clap::Parser;
//use clap::ArgAction;
//use clap::{builder::ArgPredicate, ArgAction, ValueEnum};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, env = "DEMO_HOSTNAME")]
    hostname: String,
}


fn main() {
    let args = Cli::parse();
    println!("{args:?}");
}
