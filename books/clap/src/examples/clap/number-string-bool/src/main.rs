use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value_t = 5000)]
    port: i32,

    #[arg(long, default_value_t = 0)]
    small: u8,

    #[arg(long, default_value_t = 0.0)]
    float: f32,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value = ".")]
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
