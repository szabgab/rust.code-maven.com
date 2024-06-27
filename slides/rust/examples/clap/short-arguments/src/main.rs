use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long, short, default_value_t = false)]
    debug: bool,

    #[arg(long, short='H', default_value = "127.0.0.1")]
    host: String,

    #[arg(short)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    println!("debug:   {}", args.debug);
    println!("host:    {}", args.host);
    println!("verbose: {}", args.verbose);
}


