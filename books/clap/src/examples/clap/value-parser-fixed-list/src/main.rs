use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, value_parser = ["cat", "dog", "crab"])]
    animal: String,
}

fn main() {
    let args = Cli::parse();
    println!("{args:?}");
}
