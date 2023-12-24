use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    host: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.host);
}
