use clap::Parser;

#[derive(Parser)]
#[command(about = "Hello from the code")]
struct Cli {}

fn main() {
    let _args = Cli::parse();
    println!("Use --help to show the description from the code");
}
