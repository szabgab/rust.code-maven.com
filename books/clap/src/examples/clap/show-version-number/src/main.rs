use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Cli {}

fn main() {
    let _args = Cli::parse();
    println!("Hello World!");
}
