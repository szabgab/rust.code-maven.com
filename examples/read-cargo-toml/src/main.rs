use cargo_toml::{Edition, Manifest};

fn main() {
    for filename in vec![
        "Cargo_only_edition.toml",
        "Cargo_only_version.toml",
        "Cargo_both_edition_and_version.toml",
        "Cargo_neither.toml",
    ] {
        println!("{filename}");

        let manifest = std::fs::read_to_string(filename).unwrap();
        let cargo_toml = Manifest::from_str(&manifest).unwrap();

        //println!("{cargo_toml:?}");
        match cargo_toml.package().rust_version() {
            Some(rust_version) => println!("{rust_version}"),
            None => println!("No rust-version"),
        }

        let edition = cargo_toml.package().edition();
        let year = match edition {
            Edition::E2015 => "2015",
            Edition::E2018 => "2018",
            Edition::E2021 => "2021",
        };
        println!("edition {year}");

        println!("------");
    }

    let filename = "Cargo_invalid_field.toml";

    let manifest = std::fs::read_to_string(filename).unwrap();
    let cargo_toml = Manifest::from_str(&manifest).unwrap();
    println!("{:#?}", cargo_toml);
}
