use clap::Parser;

#[derive(Parser)]
#[command(about)]
struct Cli {}

fn main() {
    let _args = Cli::parse();
    println!("Use the --help to see the description");
}
