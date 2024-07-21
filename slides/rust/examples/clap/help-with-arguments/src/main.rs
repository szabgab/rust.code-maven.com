use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, help="The name of the test")]
    test: String,

    #[arg(long, default_value="127.0.0.1", help="The name of the host")]
    host: String,

    #[arg(long, help="Debug our code")]
    debug: bool,
}

fn main() {
    let args = Cli::parse();
    println!("host: {}", args.host);
    println!("test: {}", args.test);
    println!("debug: {}", args.debug);
}


