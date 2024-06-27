use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Web {
        #[arg(long)]
        outdir: PathBuf,
    },
    Email {
        #[arg(long)]
        to: String,
    },
}

fn main() {
    let args = Cli::parse();

    println!("root: {:?}", args.root);

    match &args.command {
        Some(Commands::Web { outdir }) => {
            println!("outdir: {:?}", outdir);
        }
        Some(Commands::Email { to }) => {
            println!("to: {}", to);
        }
        None => {
            println!("There was no subcommand given");
        }
    }
}

