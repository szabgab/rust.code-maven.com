use clap::Parser;

#[derive(Parser)]
struct Cli {
    host: String,

    #[arg(required = true)]
    files: Vec<String>,
    // Probably not a good idea to put a one-value item after a vector of items
    //test: String,
    //test: Vec<String>,
}

fn main() {
    let args = Cli::parse();
    println!("host:  {}", args.host);
    println!("files: {:?}", args.files);
    //println!("test:  {}", args.test);
}
