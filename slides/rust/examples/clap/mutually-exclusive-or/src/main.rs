use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, group = "action")]
    more: bool,

    #[arg(long, group = "action")]
    less: bool,

    #[arg(short, requires = "action")]
    other: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("more: {}", cli.more);
    println!("less: {}", cli.less);
}

