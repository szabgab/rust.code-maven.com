use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    host: String,

    #[arg(long)]
    port: i32,

    #[arg(long)]
    small: u8,

    #[arg(long)]
    float: f32,

    #[arg(long)]
    debug: bool,

    #[arg(long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("host:  {}", args.host);
    println!("port   {}", args.port);
    println!("small: {}", args.small);
    println!("float: {}", args.float);
    println!("debug: {}", args.debug);
    println!("path:  {}", args.path.display());
    // `PathBuf` cannot be formatted with the default formatter; call `.display()` on it
}
