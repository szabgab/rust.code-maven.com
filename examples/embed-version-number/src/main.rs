const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let explicit_type_version: &'static str = env!("CARGO_PKG_VERSION");
    let implicit_type_version = env!("CARGO_PKG_VERSION");
    println!("Constant version {VERSION}");
    println!("Explicit version {explicit_type_version}");
    println!("Implicit version {implicit_type_version}");
}