use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, num_args=2..=3, required=true)]
    animal: Vec<String>,

    #[arg(long, num_args=3)]
    sisters: Vec<String>,
}


fn main() {
     let args = Cli::parse();
     println!("{args:?}");
}

