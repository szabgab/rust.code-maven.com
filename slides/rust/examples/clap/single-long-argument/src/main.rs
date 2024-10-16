use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    host: String,
}

fn main() {
    let args = Cli::parse();
    println!("host: {}", args.host);
}
