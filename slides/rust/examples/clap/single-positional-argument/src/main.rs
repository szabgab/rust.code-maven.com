use clap::Parser;

#[derive(Parser)]
struct Cli {
    filename: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.filename);
}

