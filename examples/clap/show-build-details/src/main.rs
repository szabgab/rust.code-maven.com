use clap::Parser;
use shadow_rs::shadow;

shadow!(build);

#[derive(Parser)]
#[command(version = build::CLAP_LONG_VERSION, about = get_detailed_version())]
struct Cli {
    #[arg(long)]
    host: String,
}


fn get_detailed_version() -> String {
     format!("
    version: {}
    branch:  {}
    sha1:    {}
    rust:    {}", build::PKG_VERSION, build::BRANCH, build::SHORT_COMMIT, build::RUST_VERSION)
 }

fn main() {
    let args = Cli::parse();
    println!("host: {}", args.host);
}
