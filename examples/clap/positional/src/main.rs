use clap::Parser;

#[derive(Parser)]
struct Cli {
    host: String,

    #[arg(required = true)]
    ports: Vec<u16>,

    #[arg(long)]
    test: String,
}

fn main() {
    let args = Cli::parse();
    println!("test:  {}", args.test);
    println!("host:  {}", args.host);
    println!("ports: {:?}", args.ports);
}
