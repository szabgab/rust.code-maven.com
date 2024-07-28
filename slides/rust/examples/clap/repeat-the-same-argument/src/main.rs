use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    animal: Vec<String>,
}


fn main() {
     let args = Cli::parse();
     println!("{args:?}");
}

