use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, value_parser = parse_even_number)]
    number: u32,
}

fn parse_even_number(number: &str) -> Result<u32, Box<dyn Error + Send + Sync + 'static>> {
    let number = number.parse::<u32>()?;
    if number % 2 == 0 {
        return Ok(number);
    }
    Err(Box::<dyn Error + Send + Sync>::from(
        "An even number is expected",
    ))
}

fn main() {
    let args = Cli::parse();
    println!("{args:?}");
}
