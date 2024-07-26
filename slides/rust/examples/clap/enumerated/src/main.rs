use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Color {
    Red,

    #[value(alias="Green", alias="verde")]
    Green,
    Blue,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    color: Color,
}


fn main() {
    let args = Cli::parse();
    println!("{args:?}");
}
