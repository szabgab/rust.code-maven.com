use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value="127.0.0.1", help="The name of the host")]
    host: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.host);
}
