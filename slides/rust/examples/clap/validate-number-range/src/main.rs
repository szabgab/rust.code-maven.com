use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, value_parser = clap::value_parser!(u16).range(1024..10000))]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("PORT = {}", cli.port);
}
