use clap::{Args, Parser};

#[derive(Parser)]
pub struct Cli {
    #[clap(flatten)]
    group: Group,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Group {
    #[clap(long)]
    more: bool,
    #[clap(long)]
    less: bool,
}

fn main() {
    let args = Cli::parse();
    println!("less: {}", args.group.less);
    println!("more: {}", args.group.more);
}
