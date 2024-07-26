use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    input: String,

    #[arg(long, value_name = "FILENAME")]
    output: String,
}

fn main() {
    let args = Cli::parse();
    println!("{args:?}");
}
