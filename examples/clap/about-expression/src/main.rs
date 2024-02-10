use clap::Parser;

#[derive(Parser)]
#[command(about = "Hello from the code")]
struct Cli {
    #[arg(long)]
    host: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.host);
}
