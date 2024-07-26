use clap::{Parser, value_parser};
use clap_complete::Shell;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, value_parser=value_parser!(Shell))]
    shell: Shell,
}


fn main() {
     let args = Cli::parse();
     println!("{args:?}");
}
