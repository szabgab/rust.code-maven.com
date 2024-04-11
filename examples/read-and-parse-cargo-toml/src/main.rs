#![allow(dead_code)]
use serde::Deserialize;
use toml::Value;


#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    edition: Option<String>,

    rust_version: Option<String>,

    #[serde(alias = "rust-version")]
    rust_dash_version: Option<String>,
}


#[derive(Deserialize, Debug)]
struct Cargo {
    package: Package,
    dependencies: Value
}


fn main() {
    let filename = "Cargo.toml";
    let content = std::fs::read_to_string(filename).unwrap();
    let parsed: Cargo = toml::from_str(&content).unwrap();
    println!("{:#?}", parsed);
    println!("name: {}", parsed.package.name);
    println!("version: {}", parsed.package.version);
    println!("edition: {:?}", parsed.package.edition);
    println!("rust_version: {:?}", parsed.package.rust_version);
    println!("rust-version: {:?}", parsed.package.rust_dash_version);

    println!("dependencies: {:?}", parsed.dependencies);
}
