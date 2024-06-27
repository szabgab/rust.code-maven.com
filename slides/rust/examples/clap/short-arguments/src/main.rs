use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    debug: bool,

    #[arg(short='H')]
    host: String,
}

fn main() {
    let args = Cli::parse();
    println!("debug:   {}", args.debug);
    println!("host:    {}", args.host);
}


